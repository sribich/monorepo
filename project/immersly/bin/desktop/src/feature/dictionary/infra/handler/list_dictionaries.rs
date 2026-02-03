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

use crate::feature::dictionary::app::use_case::list_dictionaries::ListDictionariesQuery;
use crate::system::UseCase;

//==============================================================================
// Aliases
//==============================================================================
type Req = <ListDictionariesQuery as UseCase>::Req;
type Res = <ListDictionariesQuery as UseCase>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListDictionariesRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListDictionariesResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for ListDictionariesRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {})
    }
}

impl TryFrom<Res> for ListDictionariesResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(ListDictionariesResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged)]
pub enum ListDictionariesError {
    #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<core::convert::Infallible> for ListDictionariesError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct ListDictionariesState {
    list_dictionaries_query: Arc<ListDictionariesQuery>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ListDictionariesState>,
    Json(body): Json<ListDictionariesRequest>,
) -> ApiResult<ListDictionariesResponse, ListDictionariesError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.list_dictionaries_query.run(data).await?;

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
