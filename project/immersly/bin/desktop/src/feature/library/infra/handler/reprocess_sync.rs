use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::domain::value::muid::Muid;
use railgun::error::Error;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::library::use_case::reprocess_sync::ReprocessSyncData;
use crate::feature::library::use_case::reprocess_sync::ReprocessSyncUseCase;
use crate::system::UseCase;

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
pub struct ReprocessSyncResponse(<ReprocessSyncUseCase as UseCase>::Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ReprocessSyncData> for ReprocessSyncRequest {
    type Error = Infallible;

    fn try_into(self) -> Result<ReprocessSyncData, Self::Error> {
        Ok(ReprocessSyncData {
            book_id: Muid::try_from_str(&self.book_id).unwrap(),
        })
    }
}

impl TryFrom<<ReprocessSyncUseCase as UseCase>::Res> for ReprocessSyncResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: <ReprocessSyncUseCase as UseCase>::Res) -> Result<Self, Self::Error> {
        Ok(ReprocessSyncResponse(value))
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
    reprocess_sync: Arc<ReprocessSyncUseCase>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn reprocess_sync_handler(
    State(state): State<ReprocessSyncState>,
    Json(body): Json<ReprocessSyncRequest>,
) -> ApiResult<ReprocessSyncResponse, ApiError> {
    let data: ReprocessSyncData = body.try_into()?;

    state.reprocess_sync.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, ().try_into().unwrap())
}
