#![feature(ptr_metadata, try_as_dyn, macro_metavar_expr_concat)]
use std::sync::Arc;

use app::hook::initialise_settings::InitialiseSettings;
use app::service::settings::SettingService;
use infra::repository::setting::SettingRepository;
use railgun::di::Container;
use railgun::di::InjectorBuilder;
use railgun::di::InjectorError;
use railgun::di::Routes;
use railgun::module;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use shared::infra::http::AppState;

pub mod app;
pub mod domain;
mod infra;

module!(SettingsModule, AppState);

impl Container for SettingsModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<SettingRepository>()?
            .add::<SettingService>()?
            .add::<InitialiseSettings>()?;

        Ok(())
    }
}

impl Routes<AppState> for SettingsModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
    }
}
