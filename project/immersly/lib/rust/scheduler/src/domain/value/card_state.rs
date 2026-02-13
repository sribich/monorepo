use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

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
