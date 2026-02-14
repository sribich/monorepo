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

use crate::app::dto::book::BookDto;
use crate::app::procedure::list_media::ListMediaProcedure;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(ListMediaProcedure);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListMediaRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListMediaResponse {
    books: Vec<BookDto>,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for ListMediaRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(())
    }
}

impl TryFrom<ProcedureResponse> for ListMediaResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(ListMediaResponse {
            books: value.books,
        })
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

impl From<core::convert::Infallible> for ApiError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct ListMediaState {
    list_media: Arc<ListMediaProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ListMediaState>,
    Json(body): Json<ListMediaRequest>,
) -> ApiResult<ListMediaResponse, ApiError> {
    let data: () = body.try_into()?;

    let result = state.list_media.run(()).await.unwrap();

    ApiResponse::success(StatusCode::OK, result.try_into().unwrap())
}
