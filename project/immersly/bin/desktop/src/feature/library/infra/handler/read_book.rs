use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::domain::value::muid::Muid;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::library::application::procedure::read_book::ReadBookProcedure;
use crate::handler_aliases;
use features::shared::infra::Procedure;

//==============================================================================
// Alias
//==============================================================================
handler_aliases!(ReadBookProcedure);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ReadBookRequest {
    id: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ReadBookResponse {
    text: String,
    audio_id: String,
    timestamp: Option<i64>,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for ReadBookRequest {
    type Error = Infallible;

    fn try_into(self) -> Result<Muid, Self::Error> {
        Ok(Muid::try_from_str(&self.id).unwrap())
    }
}

impl TryFrom<ProcedureResponse> for ReadBookResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(ReadBookResponse {
            text: value.text,
            audio_id: value.audio_id,
            timestamp: value.progress,
        })
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

impl From<ProcedureError> for ApiError {
    fn from(_value: ProcedureError) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct ReadBookState {
    read_book: Arc<ReadBookProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn read_book_handler(
    State(state): State<ReadBookState>,
    Json(body): Json<ReadBookRequest>,
) -> ApiResult<ReadBookResponse, ApiError> {
    let id: Muid = body.try_into()?;

    let result = state.read_book.run(id).await.unwrap();

    ApiResponse::success(StatusCode::OK, result.try_into().unwrap())
}
