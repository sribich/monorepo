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

use crate::app::procedure::get_exact_word::GetExactWordProcedure;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(GetExactWordProcedure);

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetExactWordRequest {
    word: String,
    reading: Option<String>,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetExactWordResponse(ProcedureResponse);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for GetExactWordRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            word: self.word,
            reading: self.reading,
        })
    }
}

impl TryFrom<ProcedureResponse> for GetExactWordResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(GetExactWordResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged)]
pub enum GetExactWordError {
    #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<core::convert::Infallible> for GetExactWordError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct GetExactWordState {
    query: Arc<GetExactWordProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetExactWordState>,
    Json(body): Json<GetExactWordRequest>,
) -> ApiResult<GetExactWordResponse, GetExactWordError> {
    let data: ProcedureRequest = body.try_into()?;

    let inner = state.query.run(data).await?;

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
