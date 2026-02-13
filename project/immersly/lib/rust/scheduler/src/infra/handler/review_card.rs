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
use shared::handler_aliases;
use shared::infra::Procedure;

use super::dto::review::Review;
use crate::app::procedure::review_card::ReviewCardProcedure;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(ReviewCardProcedure);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ReviewCardRequest {}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ReviewCardResponse {
    review: Option<Review>,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for ReviewCardRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {})
    }
}

impl TryFrom<ProcedureResponse> for ReviewCardResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            review: value.map(|value| Review {
                card: value.card.into(),
                next_states: value.next_states,
            }),
        })
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

impl From<ProcedureError> for ApiError {
    fn from(value: ProcedureError) -> Self {
        match value {}
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
    Json(body): Json<ReviewCardRequest>,
) -> ApiResult<ReviewCardResponse, ApiError> {
    let request = body.try_into()?;
    let response = state.procedure.run(request).await?;

    ApiResponse::success(StatusCode::OK, response.try_into()?)
}
