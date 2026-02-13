use std::sync::Arc;

use axum::extract::FromRef;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use railgun::di::Component;
use tower_http::services::ServeFile;

use crate::domain::value::ResourceId;
use crate::infra::repository::resource::ResourceRepository;

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct PlayState {
    resource_repository: Arc<ResourceRepository>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<PlayState>,
    Path(id): Path<String>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let resource = state
        .resource_repository
        .from_id(&ResourceId::try_from_str(id).unwrap())
        .await
        .unwrap()
        .unwrap();

    ServeFile::new(resource.path())
        .try_call(request)
        .await
        .unwrap()
}
