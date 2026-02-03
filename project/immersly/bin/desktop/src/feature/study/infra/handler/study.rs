use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRef, State},
    http::StatusCode,
};
use railgun::typegen::Typegen;
use railgun_api::{
    ApiError,
    json::{ApiErrorKind, ApiResponse, ApiResult},
};
use serde::{Deserialize, Serialize};

use crate::{context::study::app::query::study::StudyQuery, shared::Query};

//==============================================================================
// Aliases
//==============================================================================
type Req = <StudyQuery as Query>::Req;
type Res = <StudyQuery as Query>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct StudyRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct StudyResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for StudyRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {})
    }
}

impl TryFrom<Res> for StudyResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(StudyResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged)]
pub enum StudyError {
    #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<core::convert::Infallible> for StudyError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone)]
pub struct StudyState {
    query: Arc<StudyQuery>,
}

impl FromRef<AppState> for StudyState {
    fn from_ref(input: &AppState) -> Self {
        Self {
            query: Arc::clone(&input.study.study_query),
        }
    }
}

//==============================================================================
// Handler
//==============================================================================

pub async fn handler(
    State(state): State<StudyState>,
    Json(body): Json<StudyRequest>,
) -> ApiResult<StudyResponse, StudyError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.query.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
