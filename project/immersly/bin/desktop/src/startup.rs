use std::sync::Arc;
use std::time::Duration;

use features::shared::infra::database::Sqlite;
use features::shared::infra::http::AppState;
use features::statistics::StatisticsModule;
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

use crate::feature::analyze::AnalyzeFeature;
use crate::feature::anki_bridge::AnkiBridgeFeature;
use crate::feature::av::AudioVideoFeature;
use crate::feature::dictionary::DictionaryFeature;
use crate::feature::library::LibraryFeature;
use crate::feature::pronunciation::PronunciationFeature;
use crate::feature::scheduler::SchedulerFeature;
use crate::feature::settings::SettingsFeature;
use crate::feature::storage::StorageFeature;
use crate::system::OnStartup;
use crate::system::actor::Actor;
use crate::system::configuration::Configuration;
use crate::system::configuration::get_configuration;
use crate::system::dirs::get_app_dir;
use crate::system::hooks::LifecycleHooks;
use crate::system::http::HttpServer;
use crate::system::http::HttpServerContext;

pub trait Feature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError>;

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
    }
}

pub async fn run() -> Result<(), Box<dyn core::error::Error>> {
    let service_info = service_info!();
    let configuration = get_configuration(&service_info).unwrap();

    telemetry::init(service_info, &configuration.telemetry)?;

    let features: Vec<Box<dyn Feature>> = vec![
        SettingsFeature::new(),
        StorageFeature::new(),
        AudioVideoFeature::new(),
        DictionaryFeature::new(),
        LibraryFeature::new(),
        AnalyzeFeature::new(),
        PronunciationFeature::new(),
        AnkiBridgeFeature::new(),
        SchedulerFeature::new(),
    ];

    let modules: Vec<Box<dyn Module<State = AppState>>> = vec![StatisticsModule::new_module()];

    let injector = create_injector(&configuration, &features, &modules).await?;

    injector.run_startup_hooks().await?;

    let http_server = HttpServer::from_features(
        features,
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
    features: &[Box<dyn Feature>],
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

    for feature in features {
        feature.inject(&mut injector)?;
    }

    for module in modules {
        module
            .as_container()
            .map(|it| it.inject(&mut injector))
            .transpose()?;
    }

    Ok(injector.build())
}
