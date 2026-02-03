use features::shared::infra::http::AppState;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;

mod pick_file;

pub fn router(router: Router<AppState>, procedure: Procedure<Unresolved>) -> Router<AppState> {
    router.procedure("storage:PickFile", procedure.mutation(pick_file::handler))
}
