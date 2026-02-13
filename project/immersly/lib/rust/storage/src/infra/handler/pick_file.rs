use std::convert::Infallible;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::api::ApiError;
use railgun::api::json::ApiErrorKind;
use railgun::api::json::ApiResponse;
use railgun::api::json::ApiResult;
use railgun::typegen::Typegen;
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
    // TODO: We need to not unwrap here and instead handle the Option case for
    //       when the dialog is closed.
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
