use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use serde::Deserialize;
use serde::Serialize;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct PickFileRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct PickFileResponse {
    path: String,
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(Json(body): Json<PickFileRequest>) -> ApiResult<PickFileResponse, ApiError> {
    let res = rfd::AsyncFileDialog::new()
        // .add_filter("text", &["txt", "rs"])
        // .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file()
        .await
        .unwrap();

    ApiResponse::success(
        StatusCode::OK,
        PickFileResponse {
            path: res.path().to_str().unwrap().to_owned(),
        },
    )
}
