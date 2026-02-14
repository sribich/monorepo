use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use railgun::di::Component;
use railgun::error::Error;
use railgun::error::Location;
use railgun::typegen::Typegen;
use railgun::api::ApiError;
use railgun::api::json::ApiErrorKind;
use railgun::api::json::ApiResponse;
use railgun::api::json::ApiResult;
use serde::Deserialize;
use serde::Serialize;
use shared::domain::value::existing_file::ExistingFile;
use shared::domain::value::existing_file::ExistingFileError;
use shared::infra::Procedure;

use crate::app::procedure::import_dictionary::ImportDictionaryProcedure;
use crate::app::procedure::import_dictionary::ImportDictionaryReq;
use crate::domain::value::language_type::LanguageType;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ImportDictionaryRequest {
    /// The fully-qualified path, on disk, to the dictionary file.
    ///
    /// This path may be either the file in its native container
    /// format, or if it is possible, the "index" file of its
    /// extracted content.
    path: String,
}

//==============================================================================
// Response Payload
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct ImportDictionaryResponse {}

//==============================================================================
// Domain Parsing
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum RequestParseError {
    Path {
        #[error(impl_from)]
        source: ExistingFileError,
        location: Location,
    },
}

impl ImportDictionaryRequest {
    fn try_into_domain(self) -> Result<ExistingFile, RequestParseError> {
        ExistingFile::new(&self.path).map_err(Into::into)
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged, rename = "ImportDictionaryApiError")]
pub enum ApiError {
    #[api(
        status = "UNPROCESSABLE_ENTITY",
        code = "6ba1d621-e3f0-4a62-81c1-5005d6400ab8"
    )]
    Parse(ApiErrorKind<Option<()>>),
    #[api(
        status = "INTERNAL_SERVER_ERROR",
        code = "7edc9ab9-7f30-4cfe-812c-f4a6d5c0c22f"
    )]
    Unknown(ApiErrorKind<Option<()>>),
}

// impl From<ImportDictionaryError> for ApiError {
//     fn from(value: ImportDictionaryError) -> Self {
//         match value {
//             ImportDictionaryError::Unknown { .. } => {
//                 ApiError::Unknown(ApiErrorKind::error(value, "Internal server
// error"))             },
//         }
//     }
// }
//

impl From<RequestParseError> for ApiError {
    fn from(value: RequestParseError) -> Self {
        match value {
            RequestParseError::Path { .. } => {
                ApiError::Parse(ApiErrorKind::error(value, "Failed to parse path"))
            }
        }
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct ImportDictionaryState {
    import_dictionary: Arc<ImportDictionaryProcedure>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<ImportDictionaryState>,
    Json(body): Json<ImportDictionaryRequest>,
) -> ApiResult<ImportDictionaryResponse, ApiError> {
    let data = body.try_into_domain()?;

    state
        .import_dictionary
        .run(ImportDictionaryReq {
            dictionary_path: data,
            language_type: LanguageType::Monolingual,
        })
        .await
        .unwrap();

    ApiResponse::success(StatusCode::OK, ImportDictionaryResponse {})
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     fn get_state() -> ImportDictionaryState {
//         ImportDictionaryState {
//             import_dictionary: Arc::new(ImportDictionaryUseCase::new()),
//         }
//     }
//
//     #[tokio::test]
//     async fn test_invalid_path() {
//         let result = handler(
//             State(get_state()),
//             Json(ImportDictionaryRequest {
//                 path: "/tmp/_TESTFILE_DOES_NOT_EXIST__".into(),
//             }),
//         )
//         .await;
//
//         assert!(result.is_err_and(|err| { err.code() == "6ba1d621-e3f0-4a62-81c1-5005d6400ab8" }))
//     }
// }
