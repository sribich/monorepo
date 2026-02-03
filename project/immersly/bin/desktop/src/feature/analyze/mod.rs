use std::sync::Arc;

use features::shared::infra::http::AppState;
use infra::handler::known_words;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;

use crate::startup::Feature;

mod infra;

pub struct AnalyzeFeature {}

impl AnalyzeFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for AnalyzeFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        Ok(())
    }

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router.procedure("analyze:KnownWords", procedure.query(known_words::handler))
    }
}
