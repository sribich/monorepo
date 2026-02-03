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

use crate::feature::dictionary::app::use_case::get_exact_word::GetExactWordQuery;
use crate::system::UseCase;

//==============================================================================
// Aliases
//==============================================================================
type Req = <GetExactWordQuery as UseCase>::Req;
type Res = <GetExactWordQuery as UseCase>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetExactWordRequest {
    word: String,
    reading: Option<String>,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetExactWordResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for GetExactWordRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            word: self.word,
            reading: self.reading,
        })
    }
}

impl TryFrom<Res> for GetExactWordResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(GetExactWordResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged)]
pub enum GetExactWordError {
    #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<core::convert::Infallible> for GetExactWordError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct GetExactWordState {
    query: Arc<GetExactWordQuery>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetExactWordState>,
    Json(body): Json<GetExactWordRequest>,
) -> ApiResult<GetExactWordResponse, GetExactWordError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.query.run(data).await?;

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
