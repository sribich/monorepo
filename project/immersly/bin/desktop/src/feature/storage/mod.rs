pub mod domain;
pub mod infra;
pub mod procedure;
pub mod repository;

use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use domain::values::ResourceId;
use procedure::add_resource::AddResourceProcedure;
use procedure::commit_resource::CommitResourceProcedure;
use procedure::prepare_resource::PrepareResourceProcedure;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::Injector;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;
use repository::resource::ResourceRepository;
use tower_http::services::ServeFile;

use crate::AppState;
use crate::startup::Feature;

pub struct StorageFeature {}

impl StorageFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for StorageFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<ResourceRepository>()?
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
        infra::routes::router(router, procedure.clone())
            .apply(|router| router.route("/resource/{id}", axum::routing::get(test)))
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
