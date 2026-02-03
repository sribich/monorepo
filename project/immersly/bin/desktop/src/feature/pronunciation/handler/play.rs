use std::sync::Arc;

use axum::extract::FromRef;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun_di::Component;
use tower_http::services::ServeFile;

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct PlayState {
    db: Arc<Sqlite>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<PlayState>,
    Path(id): Path<String>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let pronunciation = state
        .db
        .client()
        .pronunciation()
        .find_unique(model::pronunciation::id::equals(
            Muid::try_from_str(id).unwrap().as_bytes().to_vec(),
        ))
        .with(model::pronunciation::resource::fetch())
        .exec()
        .await
        .unwrap()
        .unwrap();

    ServeFile::new(pronunciation.resource.unwrap().path)
        .try_call(request)
        .await
        .unwrap()
}
