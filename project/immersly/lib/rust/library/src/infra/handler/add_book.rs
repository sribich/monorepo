use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::api::ApiError;
use railgun::api::json::ApiErrorKind;
use railgun::api::json::ApiResponse;
use railgun::api::json::ApiResult;
use railgun::di::Component;
use railgun::error::Error;
use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;
use shared::domain::value::existing_file::ExistingFile;
use shared::handler_aliases;
use shared::infra::Procedure;

use crate::app::procedure::add_book::AddBookProcedure;

// ---- Handler Util------------------------------------------------------------

handler_aliases!(AddBookProcedure);

// ---- Handler Request --------------------------------------------------------

#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddBookRequest {
    title: String,
    book_path: String,
    audio_path: String,
}

impl TryInto<ProcedureRequest> for AddBookRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            title: self.title,
            book_path: ExistingFile::new(self.book_path).unwrap(),
            audio_path: ExistingFile::new(self.audio_path).unwrap(),
        })
    }
}

// ---- Handler Response -------------------------------------------------------

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddBookResponse;

impl TryFrom<ProcedureResponse> for AddBookResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(AddBookResponse)
    }
}


// ---- Error Handling ---------------------------------------------------------

#[derive(ApiError, Serialize, Typegen)]
pub enum AddBookError {}

impl From<core::convert::Infallible> for AddBookError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

// ---- Handler ----------------------------------------------------------------

#[derive(Clone, Component)]
#[component(from_state)]
pub struct AddBookState {
    add_book: Arc<AddBookProcedure>,
}

pub async fn add_book_handler(
    State(state): State<AddBookState>,
    Json(body): Json<AddBookRequest>,
) -> ApiResult<AddBookResponse, AddBookError> {
    let request = body.try_into()?;
    let response = state.add_book.run(request).await?.try_into()?;

    ApiResponse::success(StatusCode::OK, response)
}
