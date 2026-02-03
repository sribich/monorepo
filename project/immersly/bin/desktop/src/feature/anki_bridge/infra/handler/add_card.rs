use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use railgun::error::Error;
use railgun::error::Location;
use railgun::error::ResultExt;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::anki_bridge::app::use_case::add_card::AddCardUseCase;
use crate::system::UseCase;

//==============================================================================
// Type Alises
//==============================================================================
type Req = <AddCardUseCase as UseCase>::Req;
type Res = <AddCardUseCase as UseCase>::Res;
type Err = <AddCardUseCase as UseCase>::Err;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddCardRequest {
    word: String,
    reading: String,
    sentence: String,
    sentence_timestamp: (i64, i64),
    book_id: String,
    audio_id: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct AddCardResponse {}

//==============================================================================
// Parsing
//==============================================================================
#[derive(Error)]
pub enum ParseError {
    #[error(transparent)]
    Other {
        #[error(impl_from)]
        error: Box<dyn core::error::Error>,
        location: Location,
    },
}

impl TryInto<Req> for AddCardRequest {
    type Error = ParseError;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            audio_id: Muid::try_from_str(self.audio_id).boxed_local()?,
            media_id: Muid::try_from_str(self.book_id).boxed_local()?,
            word: self.word,
            reading: self.reading,
            sentence: self.sentence,
            sentence_timestamp: self.sentence_timestamp,
        })
    }
}

impl TryFrom<Res> for AddCardResponse {
    type Error = ParseError;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged, rename = "AddCardError")]
pub enum ApiError {
    #[api(
        status = "INTERNAL_SERVER_ERROR",
        code = "7edc9ab9-7f30-4cfe-812c-f4a6d5c0c22f"
    )]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<ParseError> for ApiError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Other { .. } => ApiError::Unknown(ApiErrorKind::error(value, "")),
        }
    }
}

impl From<Err> for ApiError {
    fn from(value: Err) -> Self {
        ApiError::Unknown(ApiErrorKind::error(value, ""))
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct AddCardState {
    db: Arc<Sqlite>,
    use_case: Arc<AddCardUseCase>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<AddCardState>,
    Json(body): Json<AddCardRequest>,
) -> ApiResult<AddCardResponse, ApiError> {
    let data = body.try_into()?;
    let result = state.use_case.run(data).await?;

    ApiResponse::success(StatusCode::OK, result.try_into()?)
}
