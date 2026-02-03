use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::domain::value::muid::Muid;
use railgun::error::Error;
use railgun::error::Location;
use railgun::error::ResultExt;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::library::use_case::set_progress::SetProgressUseCase;
use crate::system::UseCase;

//==============================================================================
// Type Alises
//==============================================================================
type Req = <SetProgressUseCase as UseCase>::Req;
type Res = <SetProgressUseCase as UseCase>::Res;
type Err = <SetProgressUseCase as UseCase>::Err;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct SetProgressRequest {
    media_id: String,
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

impl TryInto<Req> for SetProgressRequest {
    type Error = ParseError;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            media_id: Muid::try_from_str(self.media_id).boxed_local()?,
            progress: self.progress,
        })
    }
}

impl TryFrom<Res> for SetProgressResponse {
    type Error = ParseError;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
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

impl From<Err> for ApiError {
    fn from(value: Err) -> Self {
        ApiError::Unknown(ApiErrorKind::error(value, ""))
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct SetProgressState {
    use_case: Arc<SetProgressUseCase>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<SetProgressState>,
    Json(body): Json<SetProgressRequest>,
) -> ApiResult<SetProgressResponse, ApiError> {
    let data = body.try_into()?;
    let result = state.use_case.run(data).await?;

    ApiResponse::success(StatusCode::OK, result.try_into()?)
}
