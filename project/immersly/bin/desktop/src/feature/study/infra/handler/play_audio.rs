use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRef, FromRequest, Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
};
use futures_util::TryStreamExt;
use railgun::typegen::Typegen;
use railgun_api::{
    ApiError,
    json::{ApiErrorKind, ApiResponse, ApiResult},
};
use serde::{Deserialize, Serialize};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    context::study::app::query::play_audio::{AudioKind, PlayAudioQuery},
    domain::common::value::muid::Muid,
    shared::Query,
};

//==============================================================================
// Aliases
//==============================================================================
type Req = <PlayAudioQuery as Query>::Req;
type Res = <PlayAudioQuery as Query>::Res;

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlayAudioResponse(Res);

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<Req> for (String, AudioKind) {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<Req, Self::Error> {
        Ok(Req {
            card_id: Muid::from_str(&self.0).unwrap(),
            kind: self.1,
        })
    }
}

impl TryFrom<Res> for PlayAudioResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: Res) -> Result<Self, Self::Error> {
        Ok(PlayAudioResponse(value))
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged)]
pub enum PlayAudioError {
    #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<core::convert::Infallible> for PlayAudioError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone)]
pub struct PlayAudioState {
    query: Arc<PlayAudioQuery>,
}

impl FromRef<Arc<AppState>> for PlayAudioState {
    fn from_ref(input: &Arc<AppState>) -> Self {
        Self {
            query: Arc::clone(&input.study.play_audio_query),
        }
    }
}

//==============================================================================
// Handler
//==============================================================================
#[axum::debug_handler]
pub async fn handler(
    State(state): State<PlayAudioState>,
    Path(data): Path<(String, AudioKind)>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let data: Req = data.try_into().unwrap();

    let path = state.query.run(data).await.unwrap();
    println!("{:#?}", path);
    ServeFile::new(path).try_call(request).await.unwrap()
}
