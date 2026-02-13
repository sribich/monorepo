use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::error::Error;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::library::application::procedure::add_book::AddBookData;
use crate::feature::library::application::procedure::add_book::AddBookProcedure;
use crate::handler_aliases;
use features::shared::infra::Procedure;

//
//
//
handler_aliases!(AddBookProcedure);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddBookRequest {
    title: String,
    book_path: String,
    audio_path: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddBookResponse;

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for AddBookRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            title: self.title,
            book_path: ExistingPath::new(self.book_path).unwrap(),
            audio_path: ExistingPath::new(self.audio_path).unwrap(),
        })
    }
}

impl TryFrom<ProcedureResponse> for AddBookResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        todo!();
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
pub struct AddBookState {
    add_book: Arc<AddBookProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn add_book_handler(
    State(state): State<AddBookState>,
    Json(body): Json<AddBookRequest>,
) -> ApiResult<AddBookResponse, ApiError> {
    let data = body.try_into().unwrap();

    state.add_book.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, AddBookResponse {})
}
