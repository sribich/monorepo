use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::http::AppState;
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
use crate::feature::library::use_case::stats::StatsUseCase;
use crate::system::UseCase;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct StatsRequest {
    uuid: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse;

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum SerDeError {
    Placeholder {},
}

impl TryInto<Muid> for StatsRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Muid, Self::Error> {
        let muid = Muid::try_from_str(self.uuid).unwrap();

        Ok(muid)
    }
}

/*
impl TryFrom<()> for AddBookResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ()) -> Result<Self, Self::Error> {
        todo!();
    }
}
*/

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
#[component(from_state(AppState))]
pub struct StatsState {
    stats: Arc<StatsUseCase>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn stats_handler(
    State(state): State<StatsState>,
    Json(body): Json<StatsRequest>,
) -> ApiResult<StatsResponse, ApiError> {
    let data: Muid = body.try_into().unwrap();

    state.stats.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, StatsResponse {})
}
