use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use fsrs::{MemoryState, NextStates};
use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Dep;
use serde::{Deserialize, Serialize};

use crate::{
    context::study::domain::service::card::CardService, domain::common::value::muid::Muid,
    shared::Query,
};

#[derive(Debug, Deserialize, Typegen)]
pub struct StudyQueryRequest {}

#[derive(Debug, Serialize, Typegen)]
pub struct StudyQueryResponse {
    card: Option<Card>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct Card {
    id: String,
    word: String,
    reading: String,
    states: States,
    reading_audio: bool,
    sentence: Option<String>,
    sentence_audio: bool,
    image_id: Option<String>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct States {
    again: f32,
    hard: f32,
    good: f32,
    easy: f32,
}

impl From<NextStates> for States {
    fn from(value: NextStates) -> Self {
        Self {
            again: value.again.interval,
            hard: value.hard.interval,
            good: value.good.interval,
            easy: value.easy.interval,
        }
    }
}

pub struct StudyQuery {
    db: Arc<Sqlite>,
}

impl StudyQuery {
    pub fn new(db: Dep<Sqlite>) -> Self {
        Self { db: db.get() }
    }
}

impl Query for StudyQuery {
    type Err = core::convert::Infallible;
    type Req = StudyQueryRequest;
    type Res = StudyQueryResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;

        let card = self
            .db
            .client()
            .card()
            .find_first(vec![model::card::due::lt(now)])
            .exec()
            .await
            .unwrap();

        let card = card.map(|card| {
            let states = CardService::get_next_states(&card, now).into();

            Card {
                id: Muid::from_slice(&card.id).unwrap().to_string(),
                word: card.word,
                reading: card.reading.unwrap_or("".to_owned()),
                reading_audio: card.reading_audio.is_some(),
                sentence: card.sentence,
                sentence_audio: card.sentence_audio.is_some(),
                image_id: card
                    .image_id
                    .map(|it| Muid::from_bytes(it).unwrap().to_string()),

                states,
            }
        });

        Ok(Self::Res { card })
    }
}
