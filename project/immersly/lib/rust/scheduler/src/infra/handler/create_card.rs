use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::error::Error;
use railgun::typegen::Typegen;
use railgun::api::ApiError;
use railgun::api::json::ApiErrorKind;
use railgun::api::json::ApiResponse;
use railgun::api::json::ApiResult;
use railgun::di::Component;
use serde::Deserialize;
use serde::Serialize;
use shared::infra::Procedure;
use storage::domain::value::ResourceId;

use crate::app::procedure::create_card::AudioSegment;
use crate::app::procedure::create_card::CreateCardProcedure;

//==============================================================================
// Aliases
//==============================================================================
type ProcedureFn = CreateCardProcedure;
type ProcedureRequest = <ProcedureFn as Procedure>::Req;
type ProcedureResponse = <ProcedureFn as Procedure>::Res;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename = "CreateCardRequest", rename_all = "camelCase")]
pub struct HandlerRequest {
    word: String,
    reading: String,
    reading_audio: Option<String>,
    sentence: String,
    sentence_audio: String,
    sentence_timestamp: (u32, u32),
}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename = "CreateCardResponse", rename_all = "camelCase")]
pub struct HandlerResponse {}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum SerDeError {
    Placeholder {},
}

impl TryInto<ProcedureRequest> for HandlerRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(ProcedureRequest {
            word: self.word,
            reading: self.reading,
            reading_audio: self.reading_audio.map(|it| ResourceId::try_from_str(it).unwrap()),
            sentence: self.sentence,
            sentence_audio: Some(AudioSegment {
                resource: ResourceId::try_from_str(self.sentence_audio).unwrap().into(),
                start_time: self.sentence_timestamp.0 as usize,
                end_time: self.sentence_timestamp.1 as usize,
            }),
        })
    }
}

impl TryFrom<ProcedureResponse> for HandlerResponse {
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
    Json(body): Json<HandlerRequest>,
) -> ApiResult<HandlerResponse, ApiError> {
    let request = body.try_into()?;
    let response = state.procedure.run(request).await?;

    ApiResponse::success(StatusCode::OK, response.try_into()?)
}
