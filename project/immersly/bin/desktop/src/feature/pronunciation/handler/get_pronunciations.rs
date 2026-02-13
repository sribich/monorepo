use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::domain::value::muid::Muid;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::pronunciation::procedure::get_pronunciations::GetPronunciationsProcedure;
use crate::feature::pronunciation::procedure::get_pronunciations::Pronunciation;
use features::shared::infra::Procedure;

//==============================================================================
// Aliases
//==============================================================================
type Req = <GetPronunciationsProcedure as Procedure>::Req;
type Res = <GetPronunciationsProcedure as Procedure>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetPronunciationsRequest {
    word: String,
    // reading: String,
    speaker_id: Option<String>,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetPronunciationsResponse {
    pronunciations: Vec<Pronunciation>,
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
            speaker_id: self.speaker_id.map(|id| Muid::try_from_str(id).unwrap()),
        })
    }
}

impl TryFrom<Res> for GetPronunciationsResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(GetPronunciationsResponse {
            pronunciations: value.pronunciations,
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
