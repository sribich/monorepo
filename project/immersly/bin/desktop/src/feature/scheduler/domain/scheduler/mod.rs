use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use learning::LearningStateScheduler;
use new::NewCardScheduler;
use relearning::RelearningStateScheduler;
use review::ReviewStateScheduler;

use super::card::Card;
use super::card::CardState;
use super::card::NextCardState;
use super::state::NextStates;
use super::steps::Steps;
use super::timer::Timer;

pub mod graduated;
pub mod learning;
pub mod new;
pub mod relearning;
pub mod review;

pub const LEARNING_STEPS: &[u32] = &[1, 15, 60];
pub const RELEARNING_STEPS: &[u32] = &[30];

pub trait CardScheduler {
    fn get_next_states(&self, card: &Card) -> NextStates;

    fn get_next_state(&self, card: &Card) -> NextCardState;
}

#[derive(Debug, Clone)]
pub struct SchedulerState {
    pub desired_retention: f32,
    pub fsrs: fsrs::FSRS,

    // fsrs_states: NextStates,
    pub learning_steps: Steps,
    pub relearning_steps: Steps,
    pub timer: Timer,
}

impl SchedulerState {
    pub fn fsrs_next_states(&self, card: &Card) -> fsrs::NextStates {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // TODO: We need to handle the day transfer window. If we pass it day should be minimum 1.
        let days_elapsed = card
            .last_review
            .map(|val| since_the_epoch.abs_diff(Duration::from_secs(val)).as_secs())
            .map(|secs| (secs as f32 / 86400.0).max(1.0))
            .map(f32::round)
            .unwrap_or(0.0) as u32;

        println!("{days_elapsed:#?}");

        self.fsrs
            .next_states(card.memory_state(), self.desired_retention, days_elapsed)
            .unwrap()
    }
}

pub struct FsrsItemState {
    pub interval: f32,
    pub difficulty: f32,
    pub stability: f32,
}

impl From<&fsrs::ItemState> for FsrsItemState {
    fn from(value: &fsrs::ItemState) -> Self {
        Self {
            interval: value.interval,
            difficulty: value.memory.difficulty,
            stability: value.memory.stability,
        }
    }
}

impl FsrsItemState {
    pub fn interval_in_mins(&self) -> u32 {
        (self.interval * 1440.0) as u32
    }

    pub fn interval_in_days(&self) -> u32 {
        self.interval.round().max(1.0) as u32
    }
}

pub struct Scheduler {
    state: SchedulerState,
}

impl Scheduler {
    pub fn new(state: SchedulerState) -> Self {
        Self { state }
    }

    fn get_card_scheduler(&self, card: &Card) -> Box<dyn CardScheduler> {
        match card.state {
            CardState::New => Box::new(NewCardScheduler {}),
            CardState::Learning { step } => {
                Box::new(LearningStateScheduler::new(self.state.clone()))
            }
            CardState::Review => Box::new(ReviewStateScheduler::new(self.state.clone())),
            CardState::Relearning { step } => {
                Box::new(RelearningStateScheduler::new(self.state.clone()))
            }
            CardState::Graduated => todo!(),
        }
    }

    pub fn get_next_states(&self, card: &Card) -> NextStates {
        self.get_card_scheduler(card).get_next_states(card)
    }

    pub fn schedule_new_cards(&self, cards: Vec<Card>) -> Vec<(Card, NextCardState)> {
        assert!(
            !cards.iter().any(|it| it.state != CardState::New),
            "Card is not net"
        );

        let scheduler = NewCardScheduler {};

        cards
            .into_iter()
            .map(|card| {
                let next_state = scheduler.get_next_state(&card);

                (card, next_state)
            })
            .collect::<Vec<_>>()
    }
}

// impl From<&Card> for Box<dyn Scheduler> {}

/*
// Create a new card
    let mut card = Card::new();

    // Set desired retention
    let desired_retention = 0.9;

    // Create a new FSRS model
    let fsrs = FSRS::new(Some(&[]))?;

    // Get next states for a new card
    let next_states = fsrs.next_states(card.memory_state, desired_retention, 0)?;

    // Display the intervals for each rating
    println!(
        "Again interval: {} days",
        next_states.again.interval.round().max(1.0)
    );
    println!(
        "Hard interval: {} days",
        next_states.hard.interval.round().max(1.0)
    );
    println!(
        "Good interval: {} days",
        next_states.good.interval.round().max(1.0)
    );
    println!(
        "Easy interval: {} days",
        next_states.easy.interval.round().max(1.0)
    );

    // Assume the card was reviewed and the rating was `good`
    let next_state = next_states.good;
    let interval = next_state.interval.round().max(1.0) as u32;

    // Update the card with the new memory state and interval
    card.memory_state = Some(next_state.memory);
    card.scheduled_days = interval;
    card.last_review = Some(Utc::now());
    card.due = card.last_review.unwrap() + Duration::days(interval as i64);

    println!("Next review due: {}", card.due);
    println!("Memory state: {:?}", card.memory_state);
    Ok(())
*/
