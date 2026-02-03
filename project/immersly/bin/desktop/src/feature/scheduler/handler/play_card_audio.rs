use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use features::shared::domain::value::muid::Muid;
use railgun::typegen::Typegen;
use railgun_di::Component;
use serde::Deserialize;
use tower_http::services::ServeFile;
use tower_http::services::fs::ServeFileSystemResponseBody;

use crate::AppState;
use crate::feature::scheduler::procedure::play_card_audio::PlayCardAudioProcedure;
use crate::system::Procedure;

//==============================================================================
// Aliases
//==============================================================================
type ProcedureFn = PlayCardAudioProcedure;
type ProcedureRequest = <ProcedureFn as Procedure>::Req;
type ProcedureResponse = <ProcedureFn as Procedure>::Res;

#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "lowercase")]
pub enum AudioKind {
    Reading,
    Sentence,
    Image,
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct CreateCardState {
    procedure: Arc<ProcedureFn>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<CreateCardState>,
    Path(data): Path<(String, AudioKind)>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    if let Some(path) = state
        .procedure
        .run(ProcedureRequest {
            card_id: Muid::try_from_str(data.0).unwrap(),
            kind: data.1,
        })
        .await
        .unwrap()
    {
        println!("{:#?}", path);
        ServeFile::new(path).try_call(request).await.unwrap()
    } else {
        println!("ERR");
        axum::response::Response::builder()
            .status(404)
            .body(ServeFileSystemResponseBody::default())
            .unwrap()
    }
}
