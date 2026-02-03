use std::sync::Arc;

use features::shared::infra::http::AppState;
use forvo::client::ForvoClient;
use procedure::get_pronunciations::GetPronunciationsProcedure;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;
use repository::pronunciation::PronunciationRepository;
use service::pronunciation::PronunciationService;

use crate::startup::Feature;

pub mod domain;
mod handler;
mod infra;
pub mod procedure;
mod repository;
mod service;

pub struct PronunciationFeature {}

impl PronunciationFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for PronunciationFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        // injector.provide_with(|_| ForvoClient::new(std::env::var("FORVO_API_KEY").unwrap()));

        injector
            .add::<PronunciationRepository>()?
            .add::<PronunciationService>()?
            .add::<GetPronunciationsProcedure>()?;

        Ok(())
    }

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
            .apply(|router| {
                router.route(
                    "/pronunciation/{id}/play",
                    axum::routing::get(handler::play::handler).with_state(state),
                )
            })
            .procedure(
                "pronunciation:GetPronunciation",
                procedure.query(handler::get_pronunciations::handler),
            )
    }
}
