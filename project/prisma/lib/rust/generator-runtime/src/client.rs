use std::path::Path;
use std::sync::Arc;

use psl::parser_database::NoExtensionTypes;
use query_core::Operation;
use query_core::QueryExecutor;
pub use query_core::TxId;
use query_core::protocol::EngineProtocol;
use query_core::schema::QuerySchema;
use request_handlers::ConnectorKind;
use schema_core::commands::apply_migrations::ApplyMigrationsInput;
use schema_core::commands::apply_migrations::apply_migrations;
use schema_core::commands::{self};
use schema_core::json_rpc::types::MigrationDirectory;
use schema_core::json_rpc::types::MigrationFile;
use schema_core::json_rpc::types::MigrationList;
use schema_core::json_rpc::types::MigrationLockfile;
use schema_core::state::EngineState;
use thiserror::Error;
use tokio::fs::remove_dir_all;
use tracing::debug;

use super::RuntimeError;
use super::query::QueryError;
use super::query::QueryResult;

pub trait PrismaClient {
    fn with_tx_id(&self, tx_id: Option<TxId>) -> Self;
}

pub struct ExecutionContext {
    pub executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
    pub query_schema: Arc<QuerySchema>,
    pub url: String,
}

#[derive(Clone)]
pub(crate) enum ExecutionEngine {
    Real {
        context: Arc<ExecutionContext>,
        tx_id: Option<TxId>,
    },
}

impl ExecutionEngine {
    /// TODO: We should definitely implement a [`trace_id`] here. Check arg 4.
    async fn execute(&self, operation: Operation) -> QueryResult<serde_value::Value> {
        match self {
            Self::Real { context, tx_id } => {
                let response = context
                    .executor
                    .execute(
                        tx_id.clone(),
                        operation,
                        Arc::clone(&context.query_schema),
                        None,
                        EngineProtocol::Json,
                    )
                    .await
                    .map_err(|err| QueryError::Execute(err.into()))?;

                let data: super::prisma_value::Item = response.data.into();

                let data = serde_value::to_value(data)
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)?;

                Ok(data)
            }
        }
    }

    fn with_tx_id(&self, tx_id: Option<TxId>) -> Self {
        match self {
            ExecutionEngine::Real { context, .. } => Self::Real {
                context: Arc::clone(context),
                tx_id,
            },
        }
    }
}

#[derive(Clone)]
pub struct InternalClient {
    engine: ExecutionEngine,
}

impl InternalClient {
    pub async fn new(url: Option<String>, datamodel: &str) -> Result<Self, RuntimeError> {
        let schema = Arc::new(psl::validate(datamodel.into(), &NoExtensionTypes));
        let config = &schema.configuration;

        let source = config.datasources.first().ok_or_else(|| {
            RuntimeError::InvalidConfig("Missing datasource in schema.prisma config.".into())
        })?;

        let url = if let Some(url) = url {
            url
        } else {
            panic!("TODO dotenvy");
            /*
            source
                .load_url(|key| dotenvy::var(key).ok())
                .map_err(|_| RuntimeError::InvalidConfig("No database URL provided.".into()))?
            */
        };

        let executor = request_handlers::load_executor(
            ConnectorKind::Rust {
                url: url.clone(),
                datasource: source,
            },
            config.preview_features(),
            true,
        )
        .await
        .unwrap();
        // TODO(sr)
        // .context(eyre!("Unable to create database executor"))?;

        executor.primary_connector().get_connection().await.unwrap();
        // TODO(sr)
        // .context(eyre!("Unable to connect to database"))?;

        // source.provider
        //
        // let database_string = &context
        // .request
        // .datasources
        // .first()
        // .context(eyre!("No datasource available"))?
        // .provider;

        Ok(Self {
            engine: ExecutionEngine::Real {
                context: Arc::new(ExecutionContext {
                    executor,
                    query_schema: Arc::new(query_core::schema::build(Arc::clone(&schema), true)),
                    url,
                }),
                tx_id: None,
            },
        })
    }

    pub(crate) async fn execute(&self, operation: Operation) -> QueryResult<serde_value::Value> {
        self.engine.execute(operation).await
    }

    pub(crate) fn engine(&self) -> &ExecutionEngine {
        &self.engine
    }

    #[must_use]
    pub fn url(&self) -> String {
        match &self.engine {
            ExecutionEngine::Real { context, .. } => context.url.clone(),
        }
    }

    pub fn with_tx_id(&self, tx_id: Option<TxId>) -> Self {
        Self {
            engine: self.engine.with_tx_id(tx_id),
        }
    }
}

pub struct Migrator {
    datamodel: &'static str,
    migrations: &'static include_dir::Dir<'static>,
    url: String,
}

impl Migrator {
    pub fn new(
        datamodel: &'static str,
        migrations: &'static include_dir::Dir<'static>,
        url: String,
    ) -> Self {
        Self {
            datamodel,
            migrations,
            url,
        }
    }

    pub async fn migrate_deploy(&mut self) -> Result<(), MigrationError> {
        let datamodel = self.datamodel.to_owned();

        let temp_dir = tempfile::Builder::new()
            .prefix("prisma-migrator")
            .tempdir()
            .unwrap()
            // TODO(sr): .map_err(|_err| eyre!("Unimplemented"))?
            .into_path()
            .to_str()
            .unwrap()
            .to_owned();

        // TODO(sr): .map_err(|_err| eyre!("Unimplemented"))?;
        self.migrations.extract(&temp_dir).unwrap();

        let state = EngineState::new_from_datamodel(datamodel);

        let input = ApplyMigrationsInput {
            migrations_list: list_migrations(&Path::new(&temp_dir))?,
        };

        let output = state
            .with_connector_for_url(
                self.url.clone(),
                Box::new(|connector| Box::pin(apply_migrations(input, connector, None))),
            )
            .await
            .unwrap();
        // TODO(sr): .map_err(|err| eyre!(err));

        remove_dir_all(&temp_dir).await.unwrap(); // map_err(|_err| eyre!("Unimplemented"))?;

        // TODO: Tracing debug
        for migration in output.applied_migration_names {
            debug!("Applied migration '{migration:?}'");
        }

        tokio::time::sleep(core::time::Duration::from_millis(8)).await;

        Ok(())
    }
}

/// List the migrations present in the migration directory, lexicographically sorted by name.
///
/// If the migrations directory does not exist, it will not error but return an empty Vec.
pub fn list_migrations(migrations_directory_path: &Path) -> Result<MigrationList, MigrationError> {
    let base_dir = migrations_directory_path.to_string_lossy().into_owned();

    let lockfile = MigrationLockfile {
        path: "migration_lock.toml".to_string(),
        content: std::fs::read_to_string(migrations_directory_path.join("migration_lock.toml"))
            .ok(),
    };

    let mut entries: Vec<MigrationDirectory> = Vec::new();

    let read_dir_entries = match std::fs::read_dir(migrations_directory_path) {
        Ok(read_dir_entries) => read_dir_entries,
        Err(err) if matches!(err.kind(), std::io::ErrorKind::NotFound) => {
            return Ok(MigrationList {
                base_dir,
                lockfile,
                migration_directories: entries,
                shadow_db_init_script: "".to_owned(),
            });
        }
        Err(err) => return Err(err.into()),
    };

    for entry in read_dir_entries {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            let entry = entry.path();

            // Relative path to a migration directory from `baseDir`.
            // E.g., `20201117144659_test`.
            // This will return a &Path that is the relative path
            let entry_relative = entry
                .strip_prefix(&base_dir)
                .expect("entry is not inside base_dir");

            let path = entry_relative.to_string_lossy().into_owned();

            let migration_file = MigrationFile {
                path: "migration.sql".to_string(),
                content: std::fs::read_to_string(entry.join("migration.sql"))
                    .map_err(|_err| "Could not read migration file.".to_owned())
                    .into(),
            };

            let migration_directory = MigrationDirectory {
                path,
                migration_file,
            };
            entries.push(migration_directory);
        }
    }

    entries.sort_by(|a, b| a.migration_name().cmp(b.migration_name()));

    Ok(MigrationList {
        base_dir,
        lockfile,
        migration_directories: entries,
        shadow_db_init_script: "".to_owned(),
    })
}

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] Box<dyn core::error::Error + Send + Sync>),
}
