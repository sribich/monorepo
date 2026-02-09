//! A container to manage 0 or more schema connectors, based on request contents.
//!
//! Why this rather than using connectors directly? We must be able to use the schema engine
//! without a valid schema or database connection for commands like createDatabase and diff.

use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use enumflags2::BitFlags;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use json_rpc::types::*;
use psl::Schema;
use psl::Validated;
use psl::parser_database::NoExtensionTypes;
use psl::parser_database::SourceFile;
use psl::validate;
use schema_connector::ConnectorError;
use schema_connector::ConnectorHost;
use schema_connector::IntrospectionResult;
use schema_connector::Namespaces;
use schema_connector::SchemaConnector;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tracing_futures::Instrument;
use tracing_futures::WithSubscriber;

use crate::CoreError;
use crate::CoreResult;
use crate::SchemaContainerExt;
use crate::commands::apply_migrations::ApplyMigrationsInput;
use crate::commands::apply_migrations::ApplyMigrationsOutput;
use crate::commands::apply_migrations::apply_migrations;
use crate::commands::create_database::CreateDatabaseParams;
use crate::commands::create_database::CreateDatabaseResult;
use crate::commands::create_migration::CreateMigrationInput;
use crate::commands::create_migration::CreateMigrationOutput;
use crate::commands::create_migration::create_migration;
use crate::commands::db_execute::DbExecuteDatasourceType;
use crate::commands::db_execute::DbExecuteParams;
use crate::commands::dev_diagnostic::DevDiagnosticInput;
use crate::commands::dev_diagnostic::DevDiagnosticOutput;
use crate::commands::dev_diagnostic::dev_diagnostic;
use crate::commands::diagnose_migration_history::DiagnoseMigrationHistoryInput;
use crate::commands::diagnose_migration_history::DiagnoseMigrationHistoryOutput;
use crate::commands::diagnose_migration_history::diagnose_migration_history;
use crate::commands::diff::DiffParams;
use crate::commands::diff::DiffResult;
use crate::commands::diff::diff;
use crate::commands::ensure_connection_validity::EnsureConnectionValidityParams;
use crate::commands::ensure_connection_validity::EnsureConnectionValidityResult;
use crate::commands::evaluate_data_loss::EvaluateDataLossInput;
use crate::commands::evaluate_data_loss::EvaluateDataLossOutput;
use crate::commands::evaluate_data_loss::evaluate_data_loss;
use crate::commands::introspect::IntrospectParams;
use crate::commands::introspect::IntrospectResult;
use crate::commands::introspect::IntrospectionView;
use crate::commands::introspect_sql::IntrospectSqlParams;
use crate::commands::introspect_sql::IntrospectSqlResult;
use crate::commands::introspect_sql::SqlQueryColumnOutput;
use crate::commands::introspect_sql::SqlQueryOutput;
use crate::commands::introspect_sql::SqlQueryParameterOutput;
use crate::commands::introspect_sql::introspect_sql;
use crate::commands::mark_migration_applied::MarkMigrationAppliedInput;
use crate::commands::mark_migration_applied::MarkMigrationAppliedOutput;
use crate::commands::mark_migration_applied::mark_migration_applied;
use crate::commands::mark_migration_rolled_back::MarkMigrationRolledBackInput;
use crate::commands::mark_migration_rolled_back::MarkMigrationRolledBackOutput;
use crate::commands::mark_migration_rolled_back::mark_migration_rolled_back;
use crate::commands::reset::ResetInput;
use crate::commands::schema_push::SchemaPushInput;
use crate::commands::schema_push::SchemaPushOutput;
use crate::commands::schema_push::schema_push;
use crate::commands::version::GetDatabaseVersionInput;
use crate::extensions::ExtensionTypeConfig;
use crate::migration_schema_cache::MigrationSchemaCache;
use crate::parse_configuration_multi;

/// The container for the state of the schema engine. It can contain one or more connectors
/// corresponding to a database to be reached or that we are already connected to.
///
/// The general mechanism is that we match a single url or prisma schema to a single connector in
/// `connectors`. Each connector has its own async task, and communicates with the core through
/// channels. That ensures that each connector is handling requests one at a time to avoid
/// synchronization issues. You can think of it in terms of the actor model.
pub struct EngineState {
    // The initial Prisma schema for the engine state.
    initial_datamodel: Option<psl::ValidatedSchema>,
    host: Arc<dyn ConnectorHost>,
    extensions: Arc<ExtensionTypeConfig>,
    // A map from either:
    //
    // - a connection string / url
    // - a full schema
    //
    // To a channel leading to a spawned MigrationConnector.
    connectors: Mutex<HashMap<ConnectorRequestType, mpsc::Sender<ErasedConnectorRequest>>>,
    /// The cache for DatabaseSchemas based of migration directories to avoid redundant work during `prisma migrate dev`.
    migration_schema_cache: Arc<Mutex<MigrationSchemaCache>>,
}

impl EngineState {
    fn get_url_from_schemas(&self, container: &SchemasWithConfigDir) -> CoreResult<String> {
        let sources = container.to_psl_input();
        let (datasource, url, _, _) = parse_configuration_multi(&sources)?;

        Ok(psl::set_config_dir(
            datasource.active_connector.flavour(),
            std::path::Path::new(&container.config_dir),
            &url,
        )
        .into_owned())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ConnectorRequestType {
    Schema(Vec<(String, SourceFile)>),
    Url(String),
    InitialDatamodel,
}

impl ConnectorRequestType {
    pub fn into_connector(
        self,
        initial_datamodel: Option<&psl::ValidatedSchema>,
        config_dir: Option<&Path>,
    ) -> CoreResult<Box<dyn SchemaConnector>> {
        match self {
            Self::Schema(schemas) => crate::schema_to_connector(&schemas, config_dir),
            Self::Url(url) => {
                crate::connector_for_connection_string(url, None, BitFlags::default())
            }
            Self::InitialDatamodel => {
                if let Some(initial_datamodel) = initial_datamodel {
                    Ok(crate::initial_datamodel_to_connector(initial_datamodel)?)
                } else {
                    Err(ConnectorError::from_msg("Missing --datamodels".to_owned()))
                }
            }
        }
    }
}

/// A request from the core to a connector, in the form of an async closure.
type ConnectorRequest<O> = Box<
    dyn for<'c> FnOnce(
            &'c mut dyn SchemaConnector,
        ) -> Pin<Box<dyn Future<Output = CoreResult<O>> + Send + 'c>>
        + Send,
>;

/// Same as ConnectorRequest, but with the return type erased with a channel.
type ErasedConnectorRequest = Box<
    dyn for<'c> FnOnce(&'c mut dyn SchemaConnector) -> Pin<Box<dyn Future<Output = ()> + Send + 'c>>
        + Send
        + 'static,
>;

pub trait EngineExt {
    fn to_engine(&self) -> EngineState;
    fn into_engine(self) -> EngineState;
}

impl EngineExt for Schema<Validated> {
    fn to_engine(&self) -> EngineState {
        EngineState {
            initial_datamodel: Some(self.context().clone()),
            host: Arc::new(schema_connector::EmptyHost),
            extensions: Default::default(),
            connectors: Default::default(),
            migration_schema_cache: Arc::new(Mutex::new(Default::default())),
        }
    }

    fn into_engine(self) -> EngineState {
        EngineState {
            initial_datamodel: Some(self.into_context()),
            host: Arc::new(schema_connector::EmptyHost),
            extensions: Default::default(),
            connectors: Default::default(),
            migration_schema_cache: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl EngineState {
    pub fn new_from_schema(// initial_datamodels: Option<>,
        // host: Option<Arc<dyn ConnectorHost>>,
        // extensions: Arc<ExtensionTypeConfig>,
    ) -> Self {
        panic!();
    }

    pub fn new_from_datamodel(datamodel: String) -> Self {
        let mut datamodel = validate(datamodel.into(), &NoExtensionTypes);

        datamodel.diagnostics.to_result().unwrap();

        EngineState {
            initial_datamodel: Some(datamodel),
            host: Arc::new(schema_connector::EmptyHost),
            extensions: Default::default(),
            connectors: Default::default(),
            migration_schema_cache: Arc::new(Mutex::new(Default::default())),
        }
    }

    ///
    pub fn new(
        initial_datamodels: Option<Vec<(String, SourceFile)>>,
        host: Option<Arc<dyn ConnectorHost>>,
        extensions: Arc<ExtensionTypeConfig>,
    ) -> Self {
        let initial_datamodel = initial_datamodels
            .as_deref()
            .map(|dm| psl::validate_multi_file(dm, &*extensions));

        EngineState {
            initial_datamodel,
            host: host.unwrap_or_else(|| Arc::new(schema_connector::EmptyHost)),
            extensions,
            connectors: Default::default(),
            migration_schema_cache: Arc::new(Mutex::new(Default::default())),
        }
    }

    fn namespaces(&self) -> Option<Namespaces> {
        self.initial_datamodel
            .as_ref()
            .and_then(|schema| schema.configuration.datasources.first())
            .and_then(|ds| {
                let mut names = ds.namespaces.iter().map(|(ns, _)| ns.to_owned()).collect();
                Namespaces::from_vec(&mut names)
            })
    }

    async fn with_connector_for_request<O: Send + 'static>(
        &self,
        request: ConnectorRequestType,
        config_dir: Option<&Path>,
        f: ConnectorRequest<O>,
    ) -> CoreResult<O> {
        let (response_sender, response_receiver) = tokio::sync::oneshot::channel::<CoreResult<O>>();
        let erased: ErasedConnectorRequest = Box::new(move |connector| {
            Box::pin(async move {
                let output = f(connector).await;
                response_sender
                    .send(output)
                    .map_err(|_| ())
                    .expect("failed to send back response in schema-engine state");
            })
        });

        let mut connectors = self.connectors.lock().await;

        match connectors.get(&request) {
            Some(request_sender) => match request_sender.send(erased).await {
                Ok(()) => (),
                Err(_) => return Err(ConnectorError::from_msg("tokio mpsc send error".to_owned())),
            },
            None => {
                let request_key = request.clone();
                let mut connector =
                    request.into_connector(self.initial_datamodel.as_ref(), config_dir)?;

                connector.set_host(self.host.clone());
                let (erased_sender, mut erased_receiver) =
                    mpsc::channel::<ErasedConnectorRequest>(12);
                tokio::spawn(
                    async move {
                        while let Some(req) = erased_receiver.recv().await {
                            req(connector.as_mut()).await;
                        }
                    }
                    .with_current_subscriber(),
                );
                match erased_sender.send(erased).await {
                    Ok(()) => (),
                    Err(_) => {
                        return Err(ConnectorError::from_msg(
                            "erased sender send error".to_owned(),
                        ));
                    }
                };
                connectors.insert(request_key, erased_sender);
            }
        }

        response_receiver.await.expect("receiver boomed")
    }

    async fn with_connector_for_schema<O: Send + 'static>(
        &self,
        schemas: Vec<(String, SourceFile)>,
        config_dir: Option<&Path>,
        f: ConnectorRequest<O>,
    ) -> CoreResult<O> {
        self.with_connector_for_request::<O>(
            ConnectorRequestType::Schema(schemas.clone()),
            config_dir,
            f,
        )
        .await
    }

    /// Note: this method is used by:
    /// - `prisma db pull` via `EngineState::introspect_sql`
    /// - `prisma db execute` via `EngineState::db_execute`
    /// - `prisma/prisma tests` via `EngineState::drop_database`
    pub async fn with_connector_for_url<O: Send + 'static>(
        &self,
        url: String,
        f: ConnectorRequest<O>,
    ) -> CoreResult<O> {
        self.with_connector_for_request::<O>(ConnectorRequestType::Url(url.clone()), None, f)
            .await
    }

    async fn with_connector_from_datasource_param<O: Send + 'static>(
        &self,
        param: DatasourceParam,
        f: ConnectorRequest<O>,
    ) -> CoreResult<O> {
        match param {
            DatasourceParam::ConnectionString(UrlContainer { url }) => {
                self.with_connector_for_url(url, f).await
            }
            DatasourceParam::Schema(schemas) => {
                self.with_connector_for_schema(schemas.to_psl_input(), None, f)
                    .await
            }
        }
    }

    async fn with_default_connector<O>(&self, f: ConnectorRequest<O>) -> CoreResult<O>
    where
        O: Sized + Send + 'static,
    {
        self.with_connector_for_request::<O>(ConnectorRequestType::InitialDatamodel, None, f)
            .await
    }
}

impl EngineState {
    pub async fn apply_migrations(
        &self,
        input: ApplyMigrationsInput,
    ) -> CoreResult<ApplyMigrationsOutput> {
        let namespaces = self.namespaces();

        self.with_default_connector(Box::new(move |connector| {
            Box::pin(
                apply_migrations(input, connector, namespaces)
                    .instrument(tracing::info_span!("ApplyMigrations")),
            )
        }))
        .await
    }

    /// Create the database referenced by Prisma schema that was used to initialize the connector.
    ///
    /// TODO(sr): this currently has no tests
    /// TODO(sr): Move logic into commands dir
    pub async fn create_database(
        &self,
        params: CreateDatabaseParams,
    ) -> CoreResult<CreateDatabaseResult> {
        self.with_connector_from_datasource_param(
            params.datasource,
            Box::new(|connector| {
                Box::pin(async move {
                    let database_name = SchemaConnector::create_database(connector).await?;
                    Ok(CreateDatabaseResult { database_name })
                })
            }),
        )
        .await
    }

    pub async fn create_migration(
        &self,
        input: CreateMigrationInput,
    ) -> CoreResult<CreateMigrationOutput> {
        let migration_schema_cache: Arc<Mutex<MigrationSchemaCache>> =
            self.migration_schema_cache.clone();
        let extensions = Arc::clone(&self.extensions);
        self.with_default_connector(Box::new(move |connector| {
            let span = tracing::info_span!(
                "CreateMigration",
                migration_name = input.migration_name.as_str(),
                draft = input.draft,
            );
            Box::pin(async move {
                let mut migration_schema_cache = migration_schema_cache.lock().await;
                create_migration(input, connector, &mut migration_schema_cache, &*extensions)
                    .instrument(span)
                    .await
            })
        }))
        .await
    }

    pub async fn db_execute(&self, params: DbExecuteParams) -> CoreResult<()> {
        let url: String = match &params.datasource_type {
            DbExecuteDatasourceType::Url(UrlContainer { url }) => url.clone(),
            DbExecuteDatasourceType::Schema(schemas) => self.get_url_from_schemas(schemas)?,
        };

        self.with_connector_for_url(
            url,
            Box::new(move |connector| connector.db_execute(params.script)),
        )
        .await
    }

    pub async fn dev_diagnostic(
        &self,
        input: DevDiagnosticInput,
    ) -> CoreResult<DevDiagnosticOutput> {
        let namespaces = self.namespaces();
        let migration_schema_cache: Arc<Mutex<MigrationSchemaCache>> =
            self.migration_schema_cache.clone();
        self.with_default_connector(Box::new(move |connector| {
            Box::pin(async move {
                let mut migration_schema_cache = migration_schema_cache.lock().await;
                dev_diagnostic(input, namespaces, connector, &mut migration_schema_cache)
                    .instrument(tracing::info_span!("DevDiagnostic"))
                    .await
            })
        }))
        .await
    }

    pub async fn diagnose_migration_history(
        &self,
        input: DiagnoseMigrationHistoryInput,
    ) -> CoreResult<DiagnoseMigrationHistoryOutput> {
        let namespaces = self.namespaces();
        let migration_schema_cache: Arc<Mutex<MigrationSchemaCache>> =
            self.migration_schema_cache.clone();
        self.with_default_connector(Box::new(move |connector| {
            Box::pin(async move {
                let mut migration_schema_cache = migration_schema_cache.lock().await;
                diagnose_migration_history(
                    input,
                    namespaces,
                    connector,
                    &mut migration_schema_cache,
                )
                .instrument(tracing::info_span!("DiagnoseMigrationHistory"))
                .await
            })
        }))
        .await
    }

    pub async fn diff(&self, params: DiffParams) -> CoreResult<DiffResult> {
        diff(params, self.host.clone(), &*self.extensions).await
    }

    pub async fn drop_database(&self, url: String) -> CoreResult<()> {
        self.with_connector_for_url(
            url,
            Box::new(|connector| SchemaConnector::drop_database(connector)),
        )
        .await
    }

    pub async fn ensure_connection_validity(
        &self,
        params: EnsureConnectionValidityParams,
    ) -> CoreResult<EnsureConnectionValidityResult> {
        self.with_connector_from_datasource_param(
            params.datasource,
            Box::new(|connector| {
                Box::pin(async move {
                    SchemaConnector::ensure_connection_validity(connector).await?;
                    Ok(EnsureConnectionValidityResult {})
                })
            }),
        )
        .await
    }

    pub async fn evaluate_data_loss(
        &self,
        input: EvaluateDataLossInput,
    ) -> CoreResult<EvaluateDataLossOutput> {
        let migration_schema_cache: Arc<Mutex<MigrationSchemaCache>> =
            self.migration_schema_cache.clone();
        let extensions = Arc::clone(&self.extensions);
        self.with_default_connector(Box::new(|connector| {
            Box::pin(async move {
                let mut migration_schema_cache = migration_schema_cache.lock().await;
                evaluate_data_loss(input, connector, &mut migration_schema_cache, &*extensions)
                    .instrument(tracing::info_span!("EvaluateDataLoss"))
                    .await
            })
        }))
        .await
    }

    pub async fn introspect_sql(
        &self,
        params: IntrospectSqlParams,
    ) -> CoreResult<IntrospectSqlResult> {
        self.with_connector_for_url(
            params.url.clone(),
            Box::new(move |conn| {
                Box::pin(async move {
                    let res = introspect_sql(params, conn).await?;

                    Ok(IntrospectSqlResult {
                        queries: res
                            .queries
                            .into_iter()
                            .map(|q| SqlQueryOutput {
                                name: q.name,
                                source: q.source,
                                documentation: q.documentation,
                                parameters: q
                                    .parameters
                                    .into_iter()
                                    .map(|p| SqlQueryParameterOutput {
                                        name: p.name,
                                        typ: p.typ,
                                        documentation: p.documentation,
                                        nullable: p.nullable,
                                    })
                                    .collect(),
                                result_columns: q
                                    .result_columns
                                    .into_iter()
                                    .map(|c| SqlQueryColumnOutput {
                                        name: c.name,
                                        typ: c.typ,
                                        nullable: c.nullable,
                                    })
                                    .collect(),
                            })
                            .collect(),
                    })
                })
            }),
        )
        .await
    }

    pub async fn introspect(&self, params: IntrospectParams) -> CoreResult<IntrospectResult> {
        tracing::info!("{:?}", params.schema);
        let source_files = params.schema.to_psl_input();

        let ctx = if params.force {
            let previous_schema = psl::validate_multi_file(&source_files, &*self.extensions);

            schema_connector::IntrospectionContext::new_config_only(
                previous_schema,
                params.namespaces,
                PathBuf::new().join(&params.base_directory_path),
            )
        } else {
            psl::parse_schema_multi(&source_files, &*self.extensions).map(|previous_schema| {
                schema_connector::IntrospectionContext::new(
                    previous_schema,
                    params.namespaces,
                    PathBuf::new().join(&params.base_directory_path),
                )
            })
        }
        .map_err(ConnectorError::new_schema_parser_error)?;

        let extensions = Arc::clone(&self.extensions);
        self.with_connector_for_schema(
            source_files,
            None,
            Box::new(move |connector| {
                Box::pin(async move {
                    let IntrospectionResult {
                        datamodels,
                        views,
                        warnings,
                        is_empty,
                    } = connector.introspect(&ctx, &*extensions).await?;

                    if is_empty {
                        Err(ConnectorError::into_introspection_result_empty_error())
                    } else {
                        let views = views.map(|v| {
                            v.into_iter()
                                .map(|view| IntrospectionView {
                                    schema: view.schema,
                                    name: view.name,
                                    definition: view.definition,
                                })
                                .collect()
                        });

                        Ok(IntrospectResult {
                            schema: SchemasContainer {
                                files: datamodels
                                    .into_iter()
                                    .map(|(path, content)| SchemaContainer { path, content })
                                    .collect(),
                            },
                            views,
                            warnings,
                        })
                    }
                })
            }),
        )
        .await
    }

    pub async fn mark_migration_applied(
        &self,
        input: MarkMigrationAppliedInput,
    ) -> CoreResult<MarkMigrationAppliedOutput> {
        self.with_default_connector(Box::new(move |connector| {
            let span = tracing::info_span!(
                "MarkMigrationApplied",
                migration_name = input.migration_name.as_str()
            );
            Box::pin(mark_migration_applied(input, connector).instrument(span))
        }))
        .await
    }

    pub async fn mark_migration_rolled_back(
        &self,
        input: MarkMigrationRolledBackInput,
    ) -> CoreResult<MarkMigrationRolledBackOutput> {
        self.with_default_connector(Box::new(move |connector| {
            let span = tracing::info_span!(
                "MarkMigrationRolledBack",
                migration_name = input.migration_name.as_str()
            );
            Box::pin(mark_migration_rolled_back(input, connector).instrument(span))
        }))
        .await
    }

    pub async fn reset(&self, input: ResetInput) -> CoreResult<()> {
        tracing::debug!("Resetting the database.");
        let namespaces = self.namespaces();
        self.with_default_connector(Box::new(move |connector| {
            Box::pin(async move {
                SchemaConnector::reset(connector, false, namespaces)
                    .instrument(tracing::info_span!("Reset"))
                    .await
            })
        }))
        .await?;
        Ok(())
    }

    pub async fn schema_push(&self, input: SchemaPushInput) -> CoreResult<SchemaPushOutput> {
        let extensions = Arc::clone(&self.extensions);
        self.with_default_connector(Box::new(move |connector| {
            Box::pin(async move {
                schema_push(input, connector, &*extensions)
                    .instrument(tracing::info_span!("SchemaPush"))
                    .await
            })
        }))
        .await
    }

    pub async fn version(&self, params: Option<GetDatabaseVersionInput>) -> CoreResult<String> {
        let f: ConnectorRequest<String> = Box::new(|connector| connector.version());

        match params {
            Some(params) => {
                self.with_connector_from_datasource_param(params.datasource, f)
                    .await
            }
            None => self.with_default_connector(f).await,
        }
    }

    pub async fn dispose(&mut self) -> CoreResult<()> {
        self.connectors
            .lock()
            .await
            .drain()
            .map(|(_, snd)| async move {
                let (tx, rx) = oneshot::channel();

                snd.send({
                    Box::new(move |conn| {
                        Box::pin(async move {
                            _ = tx.send(conn.dispose().await);
                        })
                    })
                })
                .await
                .map_err(|err| {
                    CoreError::from_msg(format!(
                        "Failed to send dispose command to connector: {err}"
                    ))
                })?;

                rx.await.map_err(|err| {
                    CoreError::from_msg(format!(
                        "Connector did not respond to dispose command: {err}"
                    ))
                })?
            })
            .collect::<FuturesUnordered<_>>()
            .fold(Ok(()), async |acc, result| acc.and(result))
            .await
    }
}
