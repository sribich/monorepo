use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::api::ApiError;
use railgun::api::json::ApiErrorKind;
use railgun::api::json::ApiResponse;
use railgun::api::json::ApiResult;
use railgun::di::Component;
use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;
use shared::handler_aliases;
use shared::infra::Procedure;

use crate::app::dto::dictionary::DictionaryDto;
use crate::app::procedure::list_dictionaries::ListDictionariesProcedure;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(ListDictionariesProcedure);

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListDictionariesRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListDictionariesResponse(Vec<DictionaryDto>);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for ListDictionariesRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {})
    }
}

impl TryFrom<ProcedureResponse> for ListDictionariesResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(ListDictionariesResponse(value.dictionaries))
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
    list_dictionaries: Arc<ListDictionariesProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ListDictionariesState>,
    Json(body): Json<ListDictionariesRequest>,
) -> ApiResult<ListDictionariesResponse, ListDictionariesError> {
    let data: ProcedureRequest = body.try_into()?;
    let inner = state.list_dictionaries.run(data).await?;

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
