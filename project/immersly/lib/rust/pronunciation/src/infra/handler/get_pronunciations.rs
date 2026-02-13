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
use shared::infra::Procedure;

use crate::app::procedure::get_pronunciations::GetPronunciationsProcedure;
use crate::domain::value::PronunciationId;
use crate::infra::dto::pronunciation::PronunciationDto;

//==============================================================================
// Aliases
//==============================================================================
type Req = <GetPronunciationsProcedure as Procedure>::Req;
type Res = <GetPronunciationsProcedure as Procedure>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetPronunciationsRequest {
    word: String,
    // reading: String,
    speaker_id: Option<String>,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetPronunciationsResponse {
    pronunciations: Vec<PronunciationDto>,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for GetPronunciationsRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            word: self.word,
            // reading: self.reading,
            speaker_id: self
                .speaker_id
                .map(|id| PronunciationId::try_from_str(id).unwrap()),
        })
    }
}

impl TryFrom<Res> for GetPronunciationsResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(GetPronunciationsResponse {
            pronunciations: value
                .pronunciations
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        })
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum GetPronunciationsError {}

impl From<core::convert::Infallible> for GetPronunciationsError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct GetPronunciationsState {
    get_pronunciations: Arc<GetPronunciationsProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetPronunciationsState>,
    Json(body): Json<GetPronunciationsRequest>,
) -> ApiResult<GetPronunciationsResponse, GetPronunciationsError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.get_pronunciations.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
