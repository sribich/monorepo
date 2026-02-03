use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use railgun::typegen::Typegen;
use railgun_api::{
    ApiError,
    json::{ApiErrorKind, ApiResponse, ApiResult},
};
use serde::{Deserialize, Serialize};

use crate::{
    context::study::app::command::score::{Score, ScoreCommand},
    domain::common::value::muid::Muid,
    shared::Command,
};

//==============================================================================
// Aliases
//==============================================================================
type Req = <ScoreCommand as Command>::Req;
type Res = <ScoreCommand as Command>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct ScoreRequest {
    card_id: String,
    score: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct ScoreResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for ScoreRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            card_id: Muid::from_str(self.card_id).unwrap(),
            score: Score::from_str(&self.score),
        })
    }
}

impl TryFrom<Res> for ScoreResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(ScoreResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ScoreError {}

impl From<core::convert::Infallible> for ScoreError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone)]
pub struct ScoreState {
    command: Arc<ScoreCommand>,
}

impl FromRef<AppState> for ScoreState {
    fn from_ref(input: &AppState) -> Self {
        Self {
            command: Arc::clone(&input.study.score_command),
        }
    }
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ScoreState>,
    Json(body): Json<ScoreRequest>,
) -> ApiResult<ScoreResponse, ScoreError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.command.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
