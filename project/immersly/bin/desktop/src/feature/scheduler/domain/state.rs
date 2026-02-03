use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

use super::card::Card;
use super::card::CardState;
use super::timer::Timer;

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct NextStates {
    pub again: NextState,
    pub hard: NextState,
    pub good: NextState,
    pub easy: NextState,
}

#[derive(Debug, Typegen, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum NextState {
    Learning(LearningState),
    Review(ReviewState),
    Relearning(RelearningState),
}

impl NextState {
    pub fn apply(&self, card: &mut Card) {
        let now = chrono::Local::now();
        let timer = Timer::new();

        card.last_review = Some(now.timestamp() as u64);

        match self {
            NextState::Learning(state) => {
                card.state = CardState::Learning { step: state.step };
                card.due = timer.schedule_intraday(state.interval);
                card.difficulty = Some(f64::from(state.difficulty));
                card.stability = Some(f64::from(state.stability));
            }
            NextState::Review(state) => {
                card.state = CardState::Review;
                card.due = timer.schedule(state.interval_days);
                card.difficulty = Some(f64::from(state.difficulty));
                card.stability = Some(f64::from(state.stability));
            }
            NextState::Relearning(state) => {
                card.state = CardState::Relearning { step: state.step };
                card.due = timer.schedule_intraday(state.interval);
                card.difficulty = Some(f64::from(state.difficulty));
                card.stability = Some(f64::from(state.stability));
            }
        }
    }
}

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct LearningState {
    /// The interval, in minutes.
    pub interval: u32,
    /// The next step value. This will overflow by 1, but in that case
    /// the next time the card is scheduled it will be promoted.
    pub step: u8,
    /// The next FSRS difficulty value.
    pub difficulty: f32,
    /// The next FSRS stability value.
    pub stability: f32,
}

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct ReviewState {
    /// The interval, in days.
    pub interval_days: u32,
    /// The next FSRS difficulty value.
    pub difficulty: f32,
    /// The next FSRS stability value.
    pub stability: f32,
}

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct RelearningState {
    /// The interval, in minutes.
    pub interval: u32,
    /// The next step value. This will overflow by 1, but in that case
    /// the next time the card is scheduled it will be promoted.
    pub step: u8,
    /// The next FSRS difficulty value.
    pub difficulty: f32,
    /// The next FSRS stability value.
    pub stability: f32,
}
