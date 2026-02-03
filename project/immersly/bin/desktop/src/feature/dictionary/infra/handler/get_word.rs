use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::dictionary::app::use_case::get_word::GetWordQuery;
use crate::system::UseCase;

//==============================================================================
// Aliases
//==============================================================================
type Req = <GetWordQuery as UseCase>::Req;
type Res = <GetWordQuery as UseCase>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetWordRequest {
    word: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetWordResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for GetWordRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req { word: self.word })
    }
}

impl TryFrom<Res> for GetWordResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(GetWordResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum GetWordError {}

impl From<core::convert::Infallible> for GetWordError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct GetWordState {
    query: Arc<GetWordQuery>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetWordState>,
    Json(body): Json<GetWordRequest>,
) -> ApiResult<GetWordResponse, GetWordError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.query.run(data).await?;

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
