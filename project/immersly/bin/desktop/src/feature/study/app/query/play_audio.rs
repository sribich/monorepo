use std::sync::Arc;

use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Dep;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeFile;

use crate::{domain::common::value::muid::Muid, shared::Query};

#[derive(Debug)]
pub struct PlayAudioQueryRequest {
    pub card_id: Muid,
    pub kind: AudioKind,
}

#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "lowercase")]
pub enum AudioKind {
    Reading,
    Sentence,
}

pub struct PlayAudioQuery {
    db: Arc<Sqlite>,
}

impl PlayAudioQuery {
    pub fn new(db: Dep<Sqlite>) -> Self {
        Self { db: db.get() }
    }
}

impl Query for PlayAudioQuery {
    type Err = core::convert::Infallible;
    type Req = PlayAudioQueryRequest;
    type Res = String;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let card = self
            .db
            .client()
            .card()
            .find_unique(model::card::id::equals(data.card_id.as_bytes().to_vec()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        let path = match data.kind {
            AudioKind::Reading => card.reading_audio,
            AudioKind::Sentence => card.sentence_audio,
        };

        if let Some(path) = path {
            Ok(path)
        } else {
            todo!();
        }
    }
}

/*
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

*/
