#![feature(ptr_metadata, try_as_dyn)]
use railgun::di::Container;
use railgun::di::Routes;
use railgun::module;
use shared::infra::http::AppState;
pub use storage_app as app;
use storage_infra as infra;

module!(StorageModule, AppState);

impl Container for StorageModule {
    fn inject(
        &self,
        injector: &mut railgun::di::InjectorBuilder,
    ) -> Result<(), railgun::di::InjectorError> {
        todo!()
    }
}

impl Routes<AppState> for StorageModule {
    fn routes(
        &self,
        router: railgun::rpc::router::Router<AppState>,
        procedure: railgun::rpc::procedure::Procedure<railgun::rpc::procedure::Unresolved>,
        state: std::sync::Arc<AppState>,
    ) -> railgun::rpc::router::Router<AppState> {
        todo!()
    }
}
