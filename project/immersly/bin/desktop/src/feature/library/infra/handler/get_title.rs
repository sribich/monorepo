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

use crate::feature::library::domain::value::library_id::LibraryId;
use crate::feature::library::use_case::get_title::GetTitleProcedure;
use crate::system::Procedure;

//==============================================================================
// Aliases
//==============================================================================
type ProcedureFn = GetTitleProcedure;
type ProcedureRequest = <ProcedureFn as Procedure>::Req;
type ProcedureResponse = <ProcedureFn as Procedure>::Res;

//==============================================================================
// Request / Response
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetTitleRequest {
    title_id: String,
}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct GetTitleResponse {
    title: String,
    image_id: Option<String>,
}

//==============================================================================
// Parsing / Validation
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum SerDeError {
    Placeholder {},
}

impl TryInto<ProcedureRequest> for GetTitleRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<ProcedureRequest, Self::Error> {
        Ok(LibraryId::try_from_str(self.title_id).unwrap())
    }
}

impl TryFrom<ProcedureResponse> for GetTitleResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ProcedureResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            image_id: value.image_id.map(|it| it.to_string()),
        })
    }
}

//==============================================================================
// Error Mapping
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
pub struct GetTitleState {
    procedure: Arc<GetTitleProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<GetTitleState>,
    Json(body): Json<GetTitleRequest>,
) -> ApiResult<GetTitleResponse, ApiError> {
    let request = body.try_into()?;
    let response = state.procedure.run(request).await?;

    ApiResponse::success(StatusCode::OK, response.try_into()?)
}
