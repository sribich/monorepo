use features::shared::muid_newtype;
use fsrs::MemoryState;
use prisma_client::model;
use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::storage::domain::values::ResourceId;

muid_newtype!(CardId);

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct NextCardState {
    pub state: CardState,
    pub due: u64,
    pub stability: Option<f64>,
    pub difficulty: Option<f64>,
}

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,

    pub state: CardState,

    pub word: String,

    pub reading: String,
    pub reading_audio: Option<ResourceId>,

    pub sentence: String,
    pub sentence_audio: Option<ResourceId>,

    pub image_id: Option<ResourceId>,

    pub due: u64,
    pub last_review: Option<u64>,

    pub stability: Option<f64>,
    pub difficulty: Option<f64>,
}

impl Card {
    pub fn memory_state(&self) -> Option<MemoryState> {
        if let Some(stability) = self.stability
            && let Some(difficulty) = self.difficulty
        {
            Some(MemoryState {
                difficulty: difficulty as f32,
                stability: stability as f32,
            })
        } else {
            None
        }
    }

    pub fn step(&self) -> Option<i32> {
        match self.state {
            CardState::New => None,
            CardState::Learning { step } => Some(i32::from(step)),
            CardState::Review => None,
            CardState::Relearning { step } => Some(i32::from(step)),
            CardState::Graduated => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Typegen, Serialize, Deserialize)]
pub enum CardState {
    New,
    Learning { step: u8 },
    Review,
    Relearning { step: u8 },
    Graduated,
}

impl CardState {
    pub fn from_str(value: impl AsRef<str>, step: Option<i32>) -> Option<CardState> {
        Some(match value.as_ref() {
            "new" => CardState::New,
            "learning" => CardState::Learning {
                step: step.unwrap() as u8,
            },
            "review" => CardState::Review,
            "relearning" => CardState::Relearning {
                step: step.unwrap() as u8,
            },
            "graduated" => CardState::Graduated,
            _ => return None,
        })
    }

    pub fn to_string(&self) -> String {
        match self {
            CardState::New => "new",
            CardState::Learning { .. } => "learning",
            CardState::Review => "review",
            CardState::Relearning { .. } => "relearning",
            CardState::Graduated => "graduated",
        }
        .to_owned()
    }

    pub fn get_step(&self) -> Option<u8> {
        match self {
            CardState::New => None,
            CardState::Learning { step } => Some(*step),
            CardState::Review => None,
            CardState::Relearning { step } => Some(*step),
            CardState::Graduated => None,
        }
    }
}

impl TryFrom<model::card::Data> for Card {
    type Error = core::convert::Infallible;

    fn try_from(value: model::card::Data) -> Result<Self, Self::Error> {
        Ok(Self {
            id: CardId::from_slice_unchecked(&value.id),
            state: CardState::from_str(&value.state, value.step).unwrap(),
            word: value.word,
            reading: value.reading,
            reading_audio: value
                .reading_audio_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
            sentence: value.sentence,
            sentence_audio: value
                .sentence_audio_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
            image_id: value
                .image_id
                .map(|it| ResourceId::from_slice_unchecked(&it)),
            due: value.due as u64,
            last_review: value.last_review.map(|it| it as u64),
            stability: value.stability,
            difficulty: value.difficulty,
        })
    }
}
