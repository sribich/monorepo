use std::sync::Arc;
use std::time::Duration;

use features::dictionary::DictionaryModule;
use features::pronunciation::PronunciationModule;
use features::scheduler::SchedulerModule;
use features::shared::infra::actor::Actor;
use features::shared::infra::database::Sqlite;
use features::shared::infra::http::AppState;
use features::statistics::StatisticsModule;
use features::settings::SettingsModule;
use features::library::LibraryModule;
use features::storage::StorageModule;
use opentelemetry::global;
use opentelemetry::global::meter_provider;
use opentelemetry::metrics::InstrumentBuilder;
use railgun::core::service_info;
use railgun::di::Module;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun::telemetry;
use railgun_di::InjectionError;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;
use tokio::time::sleep;

use crate::system::configuration::Configuration;
use crate::system::configuration::get_configuration;
use crate::system::dirs::get_app_dir;
use crate::system::hooks::LifecycleHooks;
use crate::system::http::HttpServer;
use crate::system::http::HttpServerContext;

pub async fn run() -> Result<(), Box<dyn core::error::Error>> {
    let service_info = service_info!();
    let configuration = get_configuration(&service_info).unwrap();

    telemetry::init(service_info, &configuration.telemetry)?;

    let modules: Vec<Box<dyn Module<State = AppState>>> = vec![
        DictionaryModule::new_module(),
        LibraryModule::new_module(),
        PronunciationModule::new_module(),
        SchedulerModule::new_module(),
        SettingsModule::new_module(),
        StatisticsModule::new_module(),
        StorageModule::new_module(),
    ];

    let injector = create_injector(&configuration, &modules).await?;

    injector.run_startup_hooks().await?;

    let http_server = HttpServer::from_features(
        modules,
        HttpServerContext {
            port: configuration.port,
            injector: Arc::new(injector),
        },
    )
    .await;

    http_server.run().await;

    Ok(())
}

async fn create_injector(
    configuration: &Configuration,
    modules: &[Box<dyn Module<State = AppState>>],
) -> Result<Injector, Box<dyn core::error::Error>> {
    let db = {
        let app_dir = get_app_dir();
        let db_file = app_dir.join(&configuration.database_file);

        Sqlite::new(&db_file).await?
    };
    db.migrate().await?;

    let mut injector = Injector::builder();

    injector.add_value(db)?;
    injector.add_value(Actor::new())?;

    for module in modules {
        module
            .as_container()
            .map(|it| it.inject(&mut injector))
            .transpose()?;
    }

    Ok(injector.build())
}
