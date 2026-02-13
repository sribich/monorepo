#![feature(ptr_metadata, try_as_dyn)]
use std::sync::Arc;

use app::procedure::get_pronunciations::GetPronunciationsProcedure;
use app::service::pronunciation::PronunciationService;
use infra::repository::pronunciation::PronunciationRepository;
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

module!(PronunciationModule, AppState);

impl Container for PronunciationModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<PronunciationRepository>()?
            .add::<PronunciationService>()?
            .add::<GetPronunciationsProcedure>()?;

        Ok(())
    }
}

impl Routes<AppState> for PronunciationModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router.procedure(
            "pronunciation:GetPronunciation",
            procedure.query(infra::handler::get_pronunciations::handler),
        )
    }
}
