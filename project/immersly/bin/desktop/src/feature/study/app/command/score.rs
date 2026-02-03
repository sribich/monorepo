use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Dep;
use serde::{Deserialize, Serialize};

use crate::{
    context::study::domain::service::card::CardService, domain::common::value::muid::Muid,
    shared::Command,
};

pub struct ScoreCommandRequest {
    pub card_id: Muid,
    pub score: Score,
}

#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "lowercase")]
pub enum Score {
    Again,
    Hard,
    Good,
    Easy,
}

impl Score {
    pub fn to_score(&self) -> u32 {
        match self {
            Score::Again => 1,
            Score::Hard => 2,
            Score::Good => 3,
            Score::Easy => 4,
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "again" => Self::Again,
            "hard" => Self::Hard,
            "good" => Self::Good,
            "easy" => Self::Easy,
            _ => panic!("unknownv alue"),
        }
    }
}

#[derive(Debug, Serialize, Typegen)]
pub struct ScoreCommandResponse {}

pub struct ScoreCommand {
    db: Arc<Sqlite>,
}

impl ScoreCommand {
    pub fn new(db: Dep<Sqlite>) -> Self {
        Self { db: db.get() }
    }
}

impl Command for ScoreCommand {
    type Err = core::convert::Infallible;
    type Req = ScoreCommandRequest;
    type Res = ScoreCommandResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;

        let card = self
            .db
            .client()
            .card()
            .find_unique(model::card::id::equals(data.card_id.as_bytes().to_vec()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        let states = CardService::get_next_states(&card, now);
        let next_state = match data.score {
            Score::Again => states.again,
            Score::Hard => states.hard,
            Score::Good => states.good,
            Score::Easy => states.easy,
        };

        let due = now + (next_state.interval * 24.0 * 3600.0) as i64;

        let fsrs = CardService::get_fsrs();

        self.db
            .client()
            .card()
            .update(
                model::card::id::equals(data.card_id.as_bytes().to_vec()),
                vec![
                    model::card::stability::set(Some(next_state.memory.stability as f64)),
                    model::card::difficulty::set(Some(next_state.memory.difficulty as f64)),
                    model::card::last_review::set(Some(now)),
                    model::card::due::set(due),
                ],
            )
            .exec()
            .await
            .unwrap();

        self.db
            .client()
            .card_review()
            .create(
                Muid::new_now().as_bytes().to_vec(),
                now,
                data.score.to_score() as i32,
                model::card::id::equals(data.card_id.as_bytes().to_vec()),
                vec![],
            )
            .exec()
            .await
            .unwrap();

        Ok(Self::Res {})
    }
}
