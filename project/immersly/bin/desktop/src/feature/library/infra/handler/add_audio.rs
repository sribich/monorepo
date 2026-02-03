use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use features::shared::domain::value::muid::Muid;
use railgun::error::Error;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::library::use_case::add_audio::AddAudioData;
use crate::feature::library::use_case::add_audio::AddAudioUseCase;
use crate::system::UseCase;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddAudioRequest {
    media_id: String,
    path: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddAudioResponse(<AddAudioUseCase as UseCase>::Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum ParseError {
    Empty {},
}

impl TryInto<AddAudioData> for AddAudioRequest {
    type Error = Infallible;

    fn try_into(self) -> Result<AddAudioData, Self::Error> {
        Ok(AddAudioData {
            media_id: Muid::try_from_str(&self.media_id).unwrap(),
            path: ExistingPath::new(self.path).unwrap(),
        })
    }
}

impl TryFrom<<AddAudioUseCase as UseCase>::Res> for AddAudioResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: <AddAudioUseCase as UseCase>::Res) -> Result<Self, Self::Error> {
        Ok(AddAudioResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

impl From<Infallible> for ApiError {
    fn from(value: Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct AddAudioState {
    add_audio: Arc<AddAudioUseCase>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn add_audio_handler(
    State(state): State<AddAudioState>,
    Json(body): Json<AddAudioRequest>,
) -> ApiResult<AddAudioResponse, ApiError> {
    let data: AddAudioData = body.try_into()?;

    state.add_audio.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, ().try_into().unwrap())
}
