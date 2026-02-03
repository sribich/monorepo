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

use crate::feature::scheduler::procedure::schedule_cards::ScheduleCardsProcedure;
use crate::system::Procedure;

//==============================================================================
// Aliases
//==============================================================================
type ProcedureFn = ScheduleCardsProcedure;
type ProcedureRequest = <ProcedureFn as Procedure>::Req;
type ProcedureResponse = <ProcedureFn as Procedure>::Res;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename = "CreateCardRequest", rename_all = "camelCase")]
pub struct HandlerRequest {
    count: usize,
}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename = "CreateCardResponse", rename_all = "camelCase")]
pub struct HandlerResponse {
    count: usize,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for HandlerRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            count: self.count as u32,
        })
    }
}

impl TryFrom<ProcedureResponse> for HandlerResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            count: value.count as usize,
        })
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged, rename = "CreateCardError")]
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
pub struct CreateCardState {
    procedure: Arc<ProcedureFn>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<CreateCardState>,
    Json(body): Json<HandlerRequest>,
) -> ApiResult<HandlerResponse, ApiError> {
    let request = body.try_into()?;
    let response = state.procedure.run(request).await?;

    ApiResponse::success(StatusCode::OK, response.try_into()?)
}
