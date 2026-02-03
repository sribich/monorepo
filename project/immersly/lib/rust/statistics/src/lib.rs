#![feature(ptr_metadata, try_as_dyn)]

use std::sync::Arc;

use railgun::di::Routes;
use railgun::module;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use shared::infra::http::AppState;
pub use statistics_app as app;
use statistics_infra::handlers::known_words;

module!(StatisticsModule, AppState);

impl Routes<AppState> for StatisticsModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        _state: Arc<AppState>,
    ) -> Router<AppState> {
        router.procedure(
            "statistics:KnownWords",
            procedure.query(known_words::handler),
        )
    }
}
