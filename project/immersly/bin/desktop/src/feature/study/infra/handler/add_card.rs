use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRef, State},
    http::StatusCode,
};
use railgun::typegen::Typegen;
use railgun_api::{
    ApiError,
    json::{ApiErrorKind, ApiResponse, ApiResult},
};
use serde::{Deserialize, Serialize};

use crate::{
    context::study::app::command::add_card::AddCardCommand, domain::common::value::muid::Muid,
    shared::Command,
};

//==============================================================================
// Aliases
//==============================================================================
type Req = <AddCardCommand as Command>::Req;
type Res = <AddCardCommand as Command>::Res;

//==============================================================================
// Request Payload
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct AddCardRequest {
    book_id: String,
    word: String,
    reading: Option<String>,
    reading_timestamp: (u32, u32),
    sentence: String,
    sentence_timestamp: (u32, u32),
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct AddCardResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for AddCardRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            book_id: Muid::from_str(self.book_id).unwrap(),
            word: self.word,
            reading: self.reading,
            reading_timestamp: self.reading_timestamp,
            sentence: self.sentence,
            sentence_timestamp: self.sentence_timestamp,
        })
    }
}

impl TryFrom<Res> for AddCardResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(AddCardResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum AddCardError {}

impl From<core::convert::Infallible> for AddCardError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone)]
pub struct AddCardState {
    command: Arc<AddCardCommand>,
}

impl FromRef<AppState> for AddCardState {
    fn from_ref(input: &AppState) -> Self {
        Self {
            command: Arc::clone(&input.study.add_card_command),
        }
    }
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<AddCardState>,
    Json(body): Json<AddCardRequest>,
) -> ApiResult<AddCardResponse, AddCardError> {
    let data: Req = body.try_into()?;

    let inner: Res = state.command.run(data).await.unwrap();

    ApiResponse::success(StatusCode::OK, inner.try_into()?)
}
