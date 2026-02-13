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

module!(DictionaryModule, AppState);

impl Container for DictionaryModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        Ok(())
    }
}

impl Routes<AppState> for DictionaryModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
    }
}
