use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::infra::Procedure;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use super::dto::BookView;
use crate::feature::library::application::procedure::list_media::ListMedia;
use crate::handler_aliases;
use crate::system::IntoVec;

//==============================================================================
// Aliases
//==============================================================================
handler_aliases!(ListMedia);

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListMediaRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ListMediaResponse {
    books: Vec<BookView>,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<ProcedureRequest> for ListMediaRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(())
    }
}

impl TryFrom<ProcedureResponse> for ListMediaResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(ListMediaResponse {
            books: value.books.into_vec(),
        })
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
pub struct ListMediaState {
    list_media: Arc<ListMedia>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ListMediaState>,
    Json(body): Json<ListMediaRequest>,
) -> ApiResult<ListMediaResponse, ApiError> {
    let data: () = body.try_into()?;

    let result = state.list_media.run(()).await.unwrap();

    ApiResponse::success(StatusCode::OK, result.try_into().unwrap())
}
