use std::sync::Arc;

use app::hook::initialise_settings::InitialiseSettings;
use app::service::settings::SettingService;
use features::shared::infra::http::AppState;
use infra::repository::setting::SettingRepository;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;

use crate::startup::Feature;

pub mod app;
pub mod domain;
mod infra;

pub struct SettingsFeature {}

impl SettingsFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for SettingsFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<SettingRepository>()?
            .add::<SettingService>()?
            .add::<InitialiseSettings>()?;

        Ok(())
    }

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
    }
}

/*
pub fn get_settings_hooks(injector: &mut Injector) {
    injector.provide_with(|injector| {
        let mut existing = injector
            .consume::<Vec<Box<dyn OnStartup>>>()
            .unwrap_or(vec![]);

        let initialise_settings = injector.run(InitialiseSettings::new);
        existing.push(Box::new(initialise_settings));

        injector.provide(existing);
    });
}
*/
