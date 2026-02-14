use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::typegen::Typegen;
use railgun::api::ApiError;
use railgun::api::json::ApiErrorKind;
use railgun::api::json::ApiResponse;
use railgun::api::json::ApiResult;
use railgun::di::Component;
use serde::Deserialize;
use serde::Serialize;
use shared::handler_aliases;
use shared::infra::Procedure;

use crate::app::procedure::get_word::GetWordProcedure;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(GetWordProcedure);

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetWordRequest {
    word: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetWordResponse(ProcedureResponse);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for GetWordRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest { word: self.word })
    }
}

impl TryFrom<ProcedureResponse> for GetWordResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(GetWordResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum GetWordError {}

impl From<core::convert::Infallible> for GetWordError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct GetWordState {
    query: Arc<GetWordProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetWordState>,
    Json(body): Json<GetWordRequest>,
) -> ApiResult<GetWordResponse, GetWordError> {
    let data: ProcedureRequest = body.try_into()?;

    let inner = state.query.run(data).await?;

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
