use fsrs::MemoryState;
use storage::domain::value::ResourceId;

use crate::domain::value::card_id::CardId;
use crate::domain::value::card_state::CardState;

pub struct CardData {
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

pub struct Card(CardData);

impl Card {
    pub fn from_data(data: CardData) -> Self {
        Self(data)
    }

    pub fn as_data(&self) -> &CardData {
        &self.0
    }

    pub fn to_data(self) -> CardData {
        self.0
    }

    pub fn as_mut_data(&mut self) -> &mut CardData {
        &mut self.0
    }

    pub fn memory_state(&self) -> Option<MemoryState> {
        if let Some(stability) = self.0.stability
            && let Some(difficulty) = self.0.difficulty
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
        match self.0.state {
            CardState::New => None,
            CardState::Learning { step } => Some(i32::from(step)),
            CardState::Review => None,
            CardState::Relearning { step } => Some(i32::from(step)),
            CardState::Graduated => None,
        }
    }

    pub fn state(&self) -> &CardState {
        &self.0.state
    }
}
