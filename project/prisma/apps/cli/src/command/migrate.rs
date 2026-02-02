use std::{
    fs::{create_dir, create_dir_all},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use indoc::printdoc;
use psl::{IntoConfiguredExt, IntoValidatedExt, SchemaExt, Validated};
use psl_ast::SchemaParser;
// use psl::{Datasource, SchemaExt, SchemaParser};
// use psl_ast::SourceFile;
use psl_schema::Schema;

use crate::{
    command::generate::{self, GenerateArgs},
    slug::slugify,
    util::Pluralize,
};
use schema_core::{
    commands::{
        apply_migrations::{ApplyMigrationsInput, ApplyMigrationsOutput},
        create_migration::{CreateMigrationInput, CreateMigrationOutput},
        dev_diagnostic::{DevAction, DevDiagnosticInput, DevDiagnosticOutput},
        diagnose_migration_history::{DiagnoseMigrationHistoryInput, HistoryDiagnostic},
        ensure_connection_validity::EnsureConnectionValidityParams,
        evaluate_data_loss::{EvaluateDataLossInput, EvaluateDataLossOutput},
    },
    json_rpc::types::{
        DatasourceParam, MigrationDirectory, MigrationFile, MigrationList, MigrationLockfile, SchemaContainer,
        SchemasContainer,
    },
    state::{EngineExt, EngineState},
};
/*
use schema_core::{
    ExtensionTypeConfig,
    commands::{
        create_database::CreateDatabaseParams, dev_diagnostic::DevDiagnosticInput,
        diagnose_migration_history::DiagnoseMigrationHistoryInput,
        ensure_connection_validity::EnsureConnectionValidityParams,
    },
    json_rpc::types::{DatasourceParam, MigrationList, MigrationLockfile, UrlContainer},
    state::EngineState,
};
 */
use tracing::debug;

use crate::{path::diff_paths, util::print_datasource};

#[derive(Parser)]
#[command(name = "workspace")]
pub struct MigrateCli {
    #[command(subcommand)]
    command: MigrateCommand,
}

#[derive(Subcommand)]
pub enum MigrateCommand {
    #[command(about = "")]
    Deploy,
    #[command(about = "")]
    Dev,
    #[command(about = "")]
    Diff,
    #[command(about = "")]
    Reset,
    #[command(about = "")]
    Resolve,
    #[command(about = "Check the status of your database migrations")]
    Status,
}

#[derive(Parser)]
pub struct DeployArgs {}

pub async fn run(args: MigrateCli) -> Result<()> {
    match args.command {
        MigrateCommand::Deploy => migrate_deploy().await,
        MigrateCommand::Dev => migrate_dev().await,
        MigrateCommand::Diff => migrate_diff().await,
        MigrateCommand::Reset => migrate_reset().await,
        MigrateCommand::Resolve => migrate_resolve().await,
        MigrateCommand::Status => migrate_status().await,
    }
}

async fn migrate_deploy() -> Result<()> {
    // state.apply_migrations(input)
    Ok(())
}

async fn migrate_diff() -> Result<()> {
    // state.diff(params)
    Ok(())
}

async fn migrate_reset() -> Result<()> {
    // state.reset(input)
    Ok(())
}

async fn migrate_resolve() -> Result<()> {
    // state.mark_migration_applied(input)
    // state.rolled_back(input)

    Ok(())
}

/*
async fn ensure_db_exists(state: &mut EngineState, url: String) -> Result<()> {
    let result = state
        .ensure_connection_validity(EnsureConnectionValidityParams {
            datasource: DatasourceParam::ConnectionString(UrlContainer { url: url.clone() }),
        })
        .await;

    if result.is_ok() {
        return Ok(());
    }

    if result.as_ref().unwrap_err().error_code() != Some("P1003") {
        panic!("Error {:#?}", result);
        // return Err(result.unwrap_err());
    }

    let result = state
        .create_database(CreateDatabaseParams {
            datasource: DatasourceParam::ConnectionString(UrlContainer { url }),
        })
        .await;

    Ok(())
}

async fn can_connect(state: &EngineState, url: String) -> bool {
    let result = state
        .ensure_connection_validity(EnsureConnectionValidityParams {
            datasource: DatasourceParam::ConnectionString(UrlContainer { url }),
        })
        .await;

    match result {
        Ok(_) => true,
        Err(error) => {
            let code = error.error_code();
            let message = error.message();

            if let (Some(code), Some(message)) = (code, message) {
                println!("{}: {}", code, message);
            } else {
                println!("Schema engine error: {}", error.to_user_facing().message());
            }

            false
        }
    }
}
*/

async fn can_connect_to_database(state: &EngineState) -> bool {
    // state.ensure_connection_validity(EnsureConnectionValidityParams { datasource: DatasourceParam:: })

    false
    /*
    // check if we can connect to the database
    // if true: return true
    // if false: throw error
    export async function ensureCanConnectToDatabase(datasource: DataSource | undefined): Promise<Boolean | Error> {
      if (!datasource) {
        throw new Error(`A datasource block is missing in the Prisma schema file.`)
      }

      const schemaDir = path.dirname(datasource.sourceFilePath)
      const url = getConnectionUrl(datasource)

      // url exists because `ignoreEnvVarErrors: false` would have thrown an error if not
      const canConnect = await canConnectToDatabase(url, schemaDir)

      if (canConnect === true) {
        return true
      } else {
        const { code, message } = canConnect
        throw new Error(`${code}: ${message}`)
      }
    }
    */
}

trait SchemaMigrationExt {
    async fn apply_migrations(&self, engine: &EngineState) -> ApplyMigrationsOutput;
    async fn create_migration(&self, engine: &EngineState, name: String, create_only: bool) -> CreateMigrationOutput;
    async fn dev_diagnostic(&self, engine: &EngineState) -> DevDiagnosticOutput;
    async fn evaluate_data_loss(&self, engine: &EngineState) -> EvaluateDataLossOutput;

    fn to_migration_list(&self) -> MigrationList;
    fn to_schemas_container(&self) -> SchemasContainer;
}

impl SchemaMigrationExt for Schema<Validated> {
    async fn apply_migrations(&self, engine: &EngineState) -> ApplyMigrationsOutput {
        engine
            .apply_migrations(ApplyMigrationsInput {
                migrations_list: self.to_migration_list(),
            })
            .await
            .unwrap()
    }

    async fn create_migration(&self, engine: &EngineState, name: String, create_only: bool) -> CreateMigrationOutput {
        let migration = engine
            .create_migration(CreateMigrationInput {
                draft: create_only,
                migration_name: name,
                migrations_list: self.to_migration_list(),
                schema: self.to_schemas_container(),
            })
            .await
            .unwrap();

        if migration.migration_script.is_none() {
            panic!("TODO");
        }

        // Create migration folder
        let migration_list = self.to_migration_list();
        let migration_folder = PathBuf::from_str(&migration_list.base_dir)
            .unwrap()
            .join(&migration.generated_migration_name);

        if migration_folder.exists() {
            panic!(
                "The migration directory already exists at {}",
                migration_folder.to_str().unwrap()
            );
        }

        create_dir_all(&migration_folder).unwrap();

        // Write migration file
        std::fs::write(
            migration_folder.join("migration.sql"),
            migration.migration_script.as_ref().unwrap(),
        )
        .unwrap();

        migration
    }

    async fn dev_diagnostic(&self, engine: &EngineState) -> DevDiagnosticOutput {
        engine
            .dev_diagnostic(DevDiagnosticInput {
                migrations_list: self.to_migration_list(),
            })
            .await
            .unwrap()
    }

    async fn evaluate_data_loss(&self, engine: &EngineState) -> EvaluateDataLossOutput {
        engine
            .evaluate_data_loss(EvaluateDataLossInput {
                migrations_list: self.to_migration_list(),
                schema: self.to_schemas_container(),
            })
            .await
            .unwrap()
    }

    fn to_migration_list(&self) -> MigrationList {
        let migrations = self.paths().unwrap().migrations().unwrap();

        MigrationList {
            base_dir: migrations.root().to_str().unwrap().to_owned(),
            lockfile: MigrationLockfile {
                path: migrations.root().join("migrations.lock").to_str().unwrap().to_string(),
                content: None,
            },
            shadow_db_init_script: "".to_owned(),
            migration_directories: migrations
                .iter()
                .map(|migration| MigrationDirectory {
                    path: migration.subpath.clone().to_str().unwrap().to_owned(),
                    migration_file: MigrationFile {
                        path: migration.name().to_owned(),
                        content: Ok(migration.content().to_owned()),
                    },
                })
                .collect::<Vec<_>>(),
        }
    }

    fn to_schemas_container(&self) -> SchemasContainer {
        SchemasContainer {
            files: self
                .schema_files()
                .iter()
                .map(|file| SchemaContainer {
                    path: file.path().to_owned(),
                    content: file.content().to_owned(),
                })
                .collect(),
        }
    }
}

async fn migrate_dev() -> Result<()> {
    let create_only = false;
    let name = "test";

    let schema = Schema::new().parse().into_configured().into_validated();
    let engine = schema.to_engine();

    print_datasource(&schema.context().configuration);

    /// Step 1. Check for clean database
    if let DevAction::Reset(inner) = schema.dev_diagnostic(&engine).await.action {
        // TODO(sr): Expand error
        println!("{}", inner.reason);

        println!(
            "You can use {} to drop the development database.",
            "prisma migrate reset".red()
        );
        println!("{}", "All data will be lost.".bold().red());

        std::process::exit(130);
    }

    let mut applied_migrations: Vec<String> = vec![];

    /// Step 2. Apply existing migrations
    let ApplyMigrationsOutput {
        applied_migration_names,
    } = schema.apply_migrations(&engine).await;

    applied_migrations.extend(applied_migration_names);

    /// Step 3. Check for data loss
    let data_loss = schema.evaluate_data_loss(&engine).await;

    if !data_loss.unexecutable_steps.is_empty() {
        println!("\n{}", "⚠️ We found changes that cannot be executed:".bold().red());

        for item in data_loss.unexecutable_steps {
            println!("  • Step {} {}", item.step_index, item.message);
        }

        println!("");

        if !create_only {
            println!(
                "You can use {} to create the migration file, and manually modify it to address the underlying issue(s).",
                "prisma migrate dev --create-only".green()
            );

            println!(
                "Then run {} to apply it and verify it works.",
                "prisma migrate dev".green()
            );

            std::process::exit(1);
        }
    }

    if !data_loss.warnings.is_empty() {
        println!("\n{}", "⚠️ Warnings for the current datasource:".bold().yellow());

        for item in data_loss.warnings {
            println!("  • {}", item.message);
        }

        println!("\nMigration cancelled.\n");

        std::process::exit(130);
    }

    /// Step 4. Create migration
    if data_loss.migration_steps == 0 && !create_only {
        println!("Nothing to do");
        std::process::exit(0);
    }

    let name = slugify(name);

    let result = schema.create_migration(&engine, name, create_only).await;

    if create_only {
        println!(
            "Created the following migration without applying it '{}'!\n",
            result.generated_migration_name
        );
        println!(
            "You can now edit it and apply it by running {}",
            "prisma migrate dev".green()
        );

        std::process::exit(0);
    }

    /// Step 5. Apply new migration
    let ApplyMigrationsOutput {
        applied_migration_names,
    } = schema.apply_migrations(&engine).await;

    applied_migrations.extend(applied_migration_names);

    println!("");

    if applied_migrations.is_empty() {
        println!("Your database is in sync... TODO");
    } else {
        // let relative = diff_paths(migrations_dir_path, cwd);

        println!(
            "\nThe following migration(s) have been created and applied:\n\n{}\n\n{}",
            "",
            "Your database is now up to sync.".green()
        );
    }

    generate::run(GenerateArgs {}).unwrap();

    /*
        engine.dev_diagnostic(DevDiagnosticInput {
            migrations_list:
        });

        engine.apply_migrations(input);
        engine.evaluate_data_loss(input);
        engine.create_migration(input);
        engine.apply_migrations(input);
    */

    // println!("{:#?}", schema.parse_and_validate());
    // schema.parse_and_validate();

    // let schema = schema.parse();

    // println!("{:#?}", schema);

    /*
    let context = SchemaContext::load(None).unwrap();
    let config_context = context.parse::<ConfigOnlyParser>();

    let context = load_schema_context(None, None)?;

    let files = context
        .schemas
        .iter()
        .map(|path| {
            let relative_path = diff_paths(path, &context.root_dir).unwrap();
            let contents = std::fs::read_to_string(path).unwrap();

            (relative_path.to_str().unwrap().to_string(), contents.into())
        })
        .collect::<Vec<(String, SourceFile)>>();

    let mut state = EngineState::new(Some(files), None, Arc::new(ExtensionTypeConfig::default()));

    let url = config_context.context.inner.configuration.datasources[0]
        .get_connection_url()
        .clone()
        .value
        .unwrap()
        .to_owned();

    ensure_db_exists(&mut state, url).await?;

    let diagnostics = state
        .dev_diagnostic(DevDiagnosticInput {
            migrations_list: MigrationList {
                base_dir: context.root_dir.join("migrations").to_str().unwrap().to_string(),
                lockfile: MigrationLockfile {
                    path: context.root_dir.join("migrations.lock").to_str().unwrap().to_string(),
                    content: None,
                },
                shadow_db_init_script: "".to_owned(),
                migration_directories: vec![],
            },
        })
        .await;

    println!("{:#?}", diagnostics);
    */
    Ok(())
}

async fn migrate_status() -> Result<()> {
    // await loadEnvFile({ schemaPath: args['--schema'], printMessage: true, config })

    let config = Schema::new().parse_configuration().unwrap();

    print_datasource(&config);

    let engine = Schema::new().parse().into_configured().into_validated().into_engine();
    let schema = Schema::new().parse();

    let migrations = schema.paths().unwrap().migrations().unwrap();

    println!("{:#?}", migrations.root());

    let result = engine
        .diagnose_migration_history(DiagnoseMigrationHistoryInput {
            migrations_list: MigrationList {
                base_dir: migrations.root().to_str().unwrap().to_owned(),
                lockfile: MigrationLockfile {
                    path: migrations.root().join("migrations.lock").to_str().unwrap().to_string(),
                    content: None,
                },
                shadow_db_init_script: "".to_owned(),
                migration_directories: migrations
                    .iter()
                    .map(|migration| MigrationDirectory {
                        path: migration.subpath.clone().to_str().unwrap().to_owned(),
                        migration_file: MigrationFile {
                            path: migration.name().to_owned(),
                            content: Ok(migration.content().to_owned()),
                        },
                    })
                    .collect::<Vec<_>>(),
            },
            opt_in_to_shadow_database: true,
        })
        .await
        .unwrap();

    if migrations.len() > 0 {
        println!(
            "\n{} migration{} found in prisma/migrations",
            migrations.len(),
            if migrations.len() > 0 { "s" } else { "" },
        );
    } else {
        println!("No migrations found in prisma/migrations");
    }

    println!("");

    if let Some(history) = result.history {
        match history {
            HistoryDiagnostic::DatabaseIsBehind {
                unapplied_migration_names,
            } => {
                println!(
                    "The following migration{} not yet been applied:
  {}


To apply migrations in development run {}
To apply migrations in production run {}
",
                    if unapplied_migration_names.len() > 0 {
                        "s have"
                    } else {
                        " has"
                    },
                    unapplied_migration_names.join("\n  "),
                    "prisma migrate dev".bold().green(),
                    "prisma migrate deploy".bold().green(),
                );

                std::process::exit(1);
            }
            HistoryDiagnostic::MigrationsDirectoryIsBehind {
                unpersisted_migration_names,
            } => {
                todo!();
            }
            HistoryDiagnostic::HistoriesDiverge {
                last_common_migration_name,
                unpersisted_migration_names,
                unapplied_migration_names,
            } => {
                println!(
                    "Your local migration history and the migrations table from your database are different:

The last common migration is: {}

The following migration{} not yet been applied:
  {}

The migration{} from the database are not found locally in prisma/migrations:
  {}",
                    last_common_migration_name.unwrap_or("".to_string()),
                    if unapplied_migration_names.len() > 0 {
                        "s have"
                    } else {
                        " has"
                    },
                    unapplied_migration_names.join("\n  "),
                    if unpersisted_migration_names.len() > 0 {
                        "s have"
                    } else {
                        " has"
                    },
                    unpersisted_migration_names.join("\n  "),
                );

                std::process::exit(1);
            }
        }
    }

    if !result.has_migrations_table {
        println!("Database has not been baselined.");
        std::process::exit(1);
    }

    // - This is the **recovering from a partially failed migration** case.
    // - Inform the user that they can "close the case" and mark the failed migration as fixed by calling `prisma migrate resolve`.
    //     - `prisma migrate resolve --rolled-back <migration-name>` if the migration was rolled back
    //     - `prisma migrate resolve --applied <migration-name>` if the migration was rolled forward (and completed successfully)
    if !result.failed_migration_names.is_empty() {
        println!(
            "The following migration{} failed:
  {}

During development if the failed migration(s) have not been deployed to a production database you can fix the migration(s) and run {}
",
            result.failed_migration_names.pluralize("s have", " has"),
            result.failed_migration_names.join("\n  "),
            "prisma migrate dev".bold().green()
        );

        println!(
            "The failed migration(s) can be marked as rolled back or applied:

- If you rolled back the migration(s) manually:
  {}

- If you fixed the database manually (hotfix):
  {}


Read more about how to resolve migration issues in a production database:
TODO
",
            "prisma migrate resolve --rolled-back TODO_MIGRATION_NAME"
                .bold()
                .green(),
            "prisma migrate resolve --applied TODO_MIGRATION_NAME".bold().green()
        );

        std::process::exit(1);
    }

    if result.history.is_none() {
        println!("Database schema is up to date");
        std::process::exit(0);
    }

    Ok(())
}

/*

    await loadEnvFile({ schemaPath: args['--schema'], printMessage: true, config })

    const schemaContext = await loadSchemaContext({
      schemaPathFromArg: args['--schema'],
      schemaPathFromConfig: config.schema,
      schemaEngineConfig: config,
    })
    const { migrationsDirPath } = inferDirectoryConfig(schemaContext, config)

    printDatasource({ datasourceInfo: parseDatasourceInfo(schemaContext.primaryDatasource), adapter })

    const schemaFilter: MigrateTypes.SchemaFilter = {
      externalTables: config.tables?.external ?? [],
      externalEnums: config.enums?.external ?? [],
    }

    const migrate = await Migrate.setup({
      schemaEngineConfig: config,
      migrationsDirPath,
      schemaContext,
      schemaFilter,
      extensions: config['extensions'],
    })

    await ensureCanConnectToDatabase(schemaContext.primaryDatasource)

    // This is a *read-only* command (modulo shadow database).
    // - ↩️ **RPC**: ****`diagnoseMigrationHistory`, then four cases based on the response.
    //     4. Otherwise, there is no problem migrate is aware of. We could still display:
    //         - Modified since applied only relevant when using dev, they are ignored for deploy
    //         - Pending migrations (those in the migrations folder that haven't been applied yet)
    //         - If there are no pending migrations, tell the user everything looks OK and up to date.

    let diagnoseResult: EngineResults.DiagnoseMigrationHistoryOutput
    let listMigrationDirectoriesResult: EngineResults.ListMigrationDirectoriesOutput

    try {
      diagnoseResult = await migrate.diagnoseMigrationHistory({
        optInToShadowDatabase: false,
      })
      debug({ diagnoseResult: JSON.stringify(diagnoseResult, null, 2) })

      listMigrationDirectoriesResult = await migrate.listMigrationDirectories()
      debug({ listMigrationDirectoriesResult })
    } finally {
      await migrate.stop()
    }

    process.stdout.write('\n') // empty line

    if (listMigrationDirectoriesResult.migrations.length > 0) {
      const migrations = listMigrationDirectoriesResult.migrations
      process.stdout.write(
        `${migrations.length} migration${migrations.length > 1 ? 's' : ''} found in prisma/migrations\n`,
      )
    } else {
      process.stdout.write(`No migration found in prisma/migrations\n`)
    }

    let unappliedMigrations: string[] = []
    if (diagnoseResult.history?.diagnostic === 'databaseIsBehind') {
      unappliedMigrations = diagnoseResult.history.unappliedMigrationNames
      process.stdout.write(
        `Following migration${unappliedMigrations.length > 1 ? 's' : ''} have not yet been applied:
${unappliedMigrations.join('\n')}

To apply migrations in development run ${bold(green(getCommandWithExecutor(`prisma migrate dev`)))}.
To apply migrations in production run ${bold(green(getCommandWithExecutor(`prisma migrate deploy`)))}.\n`,
      )
      // Exit 1 to signal that the status is not in sync
      process.exit(1)
    } else if (diagnoseResult.history?.diagnostic === 'historiesDiverge') {
      console.error(`Your local migration history and the migrations table from your database are different:

The last common migration is: ${diagnoseResult.history.lastCommonMigrationName}

The migration${diagnoseResult.history.unappliedMigrationNames.length > 1 ? 's' : ''} have not yet been applied:
${diagnoseResult.history.unappliedMigrationNames.join('\n')}

The migration${
        diagnoseResult.history.unpersistedMigrationNames.length > 1 ? 's' : ''
      } from the database are not found locally in prisma/migrations:
${diagnoseResult.history.unpersistedMigrationNames.join('\n')}`)
      // Exit 1 to signal that the status is not in sync
      process.exit(1)
    }

    if (!diagnoseResult.hasMigrationsTable) {
      //         - This is the **baselining** case.
      //         - Look at the migrations in the migrations folder
      //             - There is no local migration
      //                 - ...and there is drift: the user is coming from db push or another migration tool.
      //                 - Guide the user to an init flow with introspect + SQL schema dump (optionally)
      //             - There are local migrations
      //                 - ↩️ **RPC** `listMigrationDirectories` ****Take the first (=oldest) migration.
      //                 - Suggest calling `prisma migrate resolve --applied <migration-name>`

      if (listMigrationDirectoriesResult.migrations.length === 0) {
        console.error(`The current database is not managed by Prisma Migrate.

Read more about how to baseline an existing production database:
${link('https://pris.ly/d/migrate-baseline')}`)
        // Exit 1 to signal that the status is not in sync
        process.exit(1)
      } else {
        const migrationId = listMigrationDirectoriesResult.migrations.shift() as string
        console.error(`The current database is not managed by Prisma Migrate.

If you want to keep the current database structure and data and create new migrations, baseline this database with the migration "${migrationId}":
${bold(green(getCommandWithExecutor(`prisma migrate resolve --applied "${migrationId}"`)))}

Read more about how to baseline an existing production database:
https://pris.ly/d/migrate-baseline`)
        // Exit 1 to signal that the status is not in sync
        process.exit(1)
      }
    } else if (diagnoseResult.failedMigrationNames.length > 0) {
      //         - This is the **recovering from a partially failed migration** case.
      //         - Inform the user that they can "close the case" and mark the failed migration as fixed by calling `prisma migrate resolve`.
      //             - `prisma migrate resolve --rolled-back <migration-name>` if the migration was rolled back
      //             - `prisma migrate resolve --applied <migration-name>` if the migration was rolled forward (and completed successfully)
      const failedMigrations = diagnoseResult.failedMigrationNames

      console.error(
        `Following migration${failedMigrations.length > 1 ? 's' : ''} have failed:
${failedMigrations.join('\n')}

During development if the failed migration(s) have not been deployed to a production database you can then fix the migration(s) and run ${bold(
          green(getCommandWithExecutor(`prisma migrate dev`)),
        )}.\n`,
      )

      console.error(`The failed migration(s) can be marked as rolled back or applied:

- If you rolled back the migration(s) manually:
${bold(green(getCommandWithExecutor(`prisma migrate resolve --rolled-back "${failedMigrations[0]}"`)))}

- If you fixed the database manually (hotfix):
${bold(green(getCommandWithExecutor(`prisma migrate resolve --applied "${failedMigrations[0]}"`)))}

Read more about how to resolve migration issues in a production database:
${link('https://pris.ly/d/migrate-resolve')}`)

      // Exit 1 to signal that the status is not in sync
      process.exit(1)
    } else {
      process.stdout.write('\n') // empty line
      if (unappliedMigrations.length === 0) {
        // Exit 0 to signal that the status is in sync
        return `Database schema is up to date!`
      }
    }

    // Only needed for the return type to match
    return ''
  }
*/
