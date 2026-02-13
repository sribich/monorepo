#![feature(ptr_metadata, try_as_dyn)]
use std::sync::Arc;

use app::procedure::add_resource::AddResourceProcedure;
use app::procedure::commit_resource::CommitResourceProcedure;
use app::procedure::extract_audio::ExtractAudioProcedure;
use app::procedure::get_resource::GetResourceProcedure;
use app::procedure::prepare_resource::PrepareResourceProcedure;
use infra::repository::resource::ResourceRepository;
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

module!(StorageModule, AppState);

impl Container for StorageModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<AddResourceProcedure>()?
            .add::<CommitResourceProcedure>()?
            .add::<ExtractAudioProcedure>()?
            .add::<GetResourceProcedure>()?
            .add::<PrepareResourceProcedure>()?
            .add::<ResourceRepository>()?;

        Ok(())
    }
}

impl Routes<AppState> for StorageModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        let router = router.apply(|router| {
            router.route(
                "/resource/{id}",
                axum::routing::get(infra::handler::play::handler).with_state(state),
            )
        });

        router.procedure(
            "storage:PickFile",
            procedure.mutation(infra::handler::pick_file::handler),
        )
    }
}

/*
pub mod domain;

use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use features::storage::domain::value::ResourceId;
use procedure::add_resource::AddResourceProcedure;
use procedure::commit_resource::CommitResourceProcedure;
use procedure::prepare_resource::PrepareResourceProcedure;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;
use tower_http::services::ServeFile;

impl Feature for StorageFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<AddResourceProcedure>()?
            .add::<CommitResourceProcedure>()?
            .add::<PrepareResourceProcedure>()?;

        Ok(())
    }

    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router.apply(|router| router.route("/resource/{id}", axum::routing::get(test)))
    }
}

pub async fn test(
    State(state): State<AppState>,
    Path(id): Path<String>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let resource = state
        .injector
        .get::<ResourceRepository>()
        .unwrap()
        .reader()
        .from_id(&ResourceId::try_from_str(&id).unwrap())
        .await
        .unwrap()
        .unwrap();

    ServeFile::new(resource.path)
        .try_call(request)
        .await
        .unwrap()
}

*/
