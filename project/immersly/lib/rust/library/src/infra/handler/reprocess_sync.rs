use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::error::Error;
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

use crate::app::procedure::reprocess_sync::ReprocessSyncProcedure;
use crate::domain::value::book_id::BookId;

//==============================================================================
// Handler Request
//==============================================================================
handler_aliases!(ReprocessSyncProcedure);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ReprocessSyncRequest {
    book_id: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ReprocessSyncResponse;

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for ReprocessSyncRequest {
    type Error = Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            book_id: BookId::try_from_str(&self.book_id).unwrap(),
        })
    }
}

impl TryFrom<ProcedureResponse> for ReprocessSyncResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(ReprocessSyncResponse)
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

impl From<Infallible> for ApiError {
    fn from(value: Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct ReprocessSyncState {
    reprocess_sync: Arc<ReprocessSyncProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ReprocessSyncState>,
    Json(body): Json<ReprocessSyncRequest>,
) -> ApiResult<ReprocessSyncResponse, ApiError> {
    let data = body.try_into()?;

    state.reprocess_sync.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, ().try_into().unwrap())
}
