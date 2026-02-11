{% set snake_name = feature_name | snake_case %}
{% set module_name = [feature_name | pascal_case, "Module"] | join(sep="") %}
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
pub use {{ snake_name }}_domain as domain;
pub use {{ snake_name }}_app as app;
use {{ snake_name }}_infra as infra;

module!({{ module_name }}, AppState);

impl Container for {{ module_name }} {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        Ok(())
    }
}

impl Routes<AppState> for {{ module_name }} {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
    }
}
