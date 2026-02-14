use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::error::Error;
use railgun::error::Location;
use railgun::error::ResultExt;
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

use crate::app::procedure::set_progress::SetProgressProcedure;
use crate::domain::value::book_id::BookId;

//==============================================================================
// Type Alises
//==============================================================================
handler_aliases!(SetProgressProcedure);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct SetProgressRequest {
    book_id: String,
    progress: i64,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct SetProgressResponse {}

//==============================================================================
// Parsing
//==============================================================================
#[derive(Error)]
pub enum ParseError {
    #[error(transparent)]
    Other {
        #[error(impl_from)]
        error: Box<dyn core::error::Error>,
        location: Location,
    },
}

impl TryInto<ProcedureRequest> for SetProgressRequest {
    type Error = ParseError;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            book_id: BookId::try_from_str(self.book_id).boxed_local()?,
            progress: self.progress,
        })
    }
}

impl TryFrom<ProcedureResponse> for SetProgressResponse {
    type Error = ParseError;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged, rename = "SetProgressError")]
pub enum ApiError {
    #[api(
        status = "INTERNAL_SERVER_ERROR",
        code = "7edc9ab9-7f30-4cfe-812c-f4a6d5c0c22f"
    )]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<ParseError> for ApiError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Other { .. } => ApiError::Unknown(ApiErrorKind::error(value, "")),
        }
    }
}

impl From<ProcedureError> for ApiError {
    fn from(value: ProcedureError) -> Self {
        ApiError::Unknown(ApiErrorKind::error(value, ""))
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct SetProgressState {
    set_progress: Arc<SetProgressProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<SetProgressState>,
    Json(body): Json<SetProgressRequest>,
) -> ApiResult<SetProgressResponse, ApiError> {
    let data = body.try_into()?;
    let result = state.set_progress.run(data).await?;

    ApiResponse::success(StatusCode::OK, result.try_into()?)
}
