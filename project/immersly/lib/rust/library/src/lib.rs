#![feature(ptr_metadata, try_as_dyn)]
use std::sync::Arc;

use railgun::di::Container;
use railgun::di::InjectorBuilder;
use railgun::di::InjectorError;
use railgun::di::Routes;
use railgun::module;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use shared::infra::http::AppState;
pub use library_app as app;
use library_infra as infra;

module!(LibraryModule, AppState);

impl Container for LibraryModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        Ok(())
    }
}

impl Routes<AppState> for LibraryModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
    }
}
