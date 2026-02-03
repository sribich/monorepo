use std::convert::Infallible;
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

use crate::feature::library::application::procedure::get_media::GetMedia;
use crate::handler_aliases;
use crate::system::Procedure;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(GetMedia);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaRequest {
    id: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaResponse;

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for GetMediaRequest {
    type Error = Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(Muid::try_from_str(&self.id).unwrap())
    }
}

impl TryFrom<ProcedureResponse> for GetMediaResponse {
    type Error = Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(
            GetMediaResponse, /*MediaDto {
                                          id: value.id,
                                          title: value.title,
                                          // kind: value.kind,
                                      })
                              */
        )
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
#[derive(Component)]
#[component(from_state)]
pub struct GetMediaState {
    get_media: Arc<GetMedia>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetMediaState>,
    Json(body): Json<GetMediaRequest>,
) -> ApiResult<GetMediaResponse, ApiError> {
    let id: Muid = body.try_into()?;

    let result = state.get_media.run(id).await.unwrap();

    ApiResponse::success(StatusCode::OK, result.try_into().unwrap())
}
