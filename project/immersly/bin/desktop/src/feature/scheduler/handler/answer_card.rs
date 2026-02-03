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

use crate::feature::scheduler::domain::card::CardId;
use crate::feature::scheduler::domain::state::NextState;
use crate::feature::scheduler::procedure::answer_card::AnswerCardProcedure;
use crate::system::Procedure;

//==============================================================================
// Aliases
//==============================================================================
type ProcedureFn = AnswerCardProcedure;
type ProcedureRequest = <ProcedureFn as Procedure>::Req;
type ProcedureResponse = <ProcedureFn as Procedure>::Res;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AnswerCardRequest {
    card_id: String,
    next_state: NextState,
}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AnswerCardResponse {}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum SerDeError {
    Placeholder {},
}

impl TryInto<ProcedureRequest> for AnswerCardRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            id: CardId::try_from_str(&self.card_id).unwrap(),
            answer: self.next_state,
        })
    }
}

impl TryFrom<ProcedureResponse> for AnswerCardResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(Self {})
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
pub struct CreateCardState {
    procedure: Arc<ProcedureFn>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<CreateCardState>,
    Json(body): Json<AnswerCardRequest>,
) -> ApiResult<AnswerCardResponse, ApiError> {
    let request = body.try_into()?;
    let response = state.procedure.run(request).await?;

    ApiResponse::success(StatusCode::OK, response.try_into()?)
}
