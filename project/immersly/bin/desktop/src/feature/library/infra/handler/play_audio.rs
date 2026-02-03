use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Component;
use serde::Deserialize;
use tower_http::services::ServeFile;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct PlayAudioRequest {
    id: String,
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct PlayAudioState {
    db: Arc<Sqlite>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn play_audio_handler(
    State(state): State<PlayAudioState>,
    Path(id): Path<String>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let path = state
        .db
        .client()
        .book()
        .find_unique(model::book::id::equals(
            Muid::try_from_str(&id).unwrap().as_bytes().to_vec(),
        ))
        .with(model::book::audio_resource::fetch())
        .exec()
        .await
        .unwrap()
        .unwrap()
        .audio_resource
        .unwrap()
        .unwrap()
        .path;

    ServeFile::new(path).try_call(request).await.unwrap()
}
