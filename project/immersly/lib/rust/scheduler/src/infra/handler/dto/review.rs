use railgun::typegen::Typegen;
use serde::Serialize;

use crate::domain::entity::card::Card;
use crate::domain::state::NextStates;
use crate::domain::value::card_state::CardState;

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub card: CardDto,
    pub next_states: NextStates,
}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename = "Card", rename_all = "camelCase")]
pub struct CardDto {
    pub id: String,

    pub state: CardState,

    pub word: String,

    pub reading: String,
    pub reading_audio: Option<String>,

    pub sentence: String,
    pub sentence_audio: Option<String>,

    pub image_id: Option<String>,

    pub due: u64,
    pub last_review: Option<u64>,

    pub stability: Option<f64>,
    pub difficulty: Option<f64>,
}

impl From<Card> for CardDto {
    fn from(value: Card) -> Self {
        let value = value.to_data();

        CardDto {
            id: value.id.to_string(),
            state: value.state,
            word: value.word,
            reading: value.reading,
            reading_audio: value.reading_audio.map(|it| it.to_string()),
            sentence: value.sentence,
            sentence_audio: value.sentence_audio.map(|it| it.to_string()),
            image_id: value.image_id.map(|it| it.to_string()),
            due: value.due,
            last_review: value.last_review,
            stability: value.stability,
            difficulty: value.difficulty,
        }
    }
}
