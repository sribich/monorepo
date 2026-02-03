use std::sync::Arc;

use app::service::anki::AnkiService;
use app::service::media::AnkiMediaService;
use app::use_case::add_card::AddCardUseCase;
use features::shared::infra::http::AppState;
use infra::handler::add_card;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;

use crate::startup::Feature;

pub mod app;
pub mod domain;
mod infra;

pub struct AnkiBridgeFeature {}

impl AnkiBridgeFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for AnkiBridgeFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add_value(AnkiService::new())?
            .add::<AnkiMediaService>()?
            .add::<AddCardUseCase>()?;

        Ok(())
    }

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router.procedure("anki:addCard", procedure.mutation(add_card::handler))
    }
}
