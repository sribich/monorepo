use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::CardScheduler;
use super::FsrsItemState;
use super::SchedulerState;
use crate::domain::entity::card::Card;
use crate::domain::state::NextState;
use crate::domain::state::NextStates;
use crate::domain::state::RelearningState;
use crate::domain::state::ReviewState;
use crate::domain::value::card_state::CardState;
use crate::domain::value::next_card_state::NextCardState;

pub struct RelearningStateScheduler {
    state: SchedulerState,
}

impl RelearningStateScheduler {
    pub fn new(state: SchedulerState) -> Self {
        Self { state }
    }

    pub fn get_step(&self, card: &Card) -> u8 {
        if let CardState::Relearning { step } = card.as_data().state {
            step
        } else {
            panic!("incorrect card state {:#?}", card.as_data().state);
        }
    }
}

impl CardScheduler for RelearningStateScheduler {
    fn get_next_states(&self, card: &Card) -> NextStates {
        let fsrs_states = self.state.fsrs_next_states(card);

        NextStates {
            again: self.again(card, &self.state, &fsrs_states),
            hard: self.hard(card, &self.state, &fsrs_states),
            good: self.good(card, &self.state, &fsrs_states),
            easy: self.easy(card, &self.state, &fsrs_states),
        }
    }

    fn get_next_state(&self, card: &Card) -> NextCardState {
        assert!(
            !(card.as_data().state != CardState::New),
            "Unexpected card state"
        );

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        NextCardState {
            state: CardState::Learning { step: 1 },
            due: since_the_epoch.as_secs(),
            stability: None,
            difficulty: None,
        }
    }
}

impl RelearningStateScheduler {
    fn again(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.again).into();
        let step = self.get_step(card);

        if let Some(mins) = state.relearning_steps.nth_as_mins(0) {
            return NextState::Relearning(RelearningState {
                interval: mins,
                step: 1,
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            });
        }

        if self
            .state
            .timer
            .can_schedule_intraday(item_state.interval_in_mins())
        {
            NextState::Relearning(RelearningState {
                interval: item_state.interval_in_mins(),
                step,
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            })
        } else {
            NextState::Review(ReviewState {
                interval_days: item_state.interval_in_days(),
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            })
        }
    }

    fn hard(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.hard).into();

        let step = self.get_step(card);
        assert!(step >= 1);

        if let (Some(prev), Some(curr)) = (
            state.relearning_steps.nth_as_mins((step - 1) as usize),
            state.relearning_steps.nth_as_mins(step as usize),
        ) {
            return NextState::Relearning(RelearningState {
                interval: u32::midpoint(prev, curr),
                step,
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            });
        }

        if self
            .state
            .timer
            .can_schedule_intraday(item_state.interval_in_mins())
        {
            NextState::Relearning(RelearningState {
                interval: item_state.interval_in_mins(),
                step,
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            })
        } else {
            NextState::Review(ReviewState {
                interval_days: item_state.interval_in_days(),
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            })
        }
    }

    fn good(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.good).into();

        let step = self.get_step(card);

        if let Some(mins) = state.relearning_steps.nth_as_mins(step as usize) {
            return NextState::Relearning(RelearningState {
                interval: mins,
                step: step + 1,
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            });
        }

        if self
            .state
            .timer
            .can_schedule_intraday(item_state.interval_in_mins())
        {
            NextState::Relearning(RelearningState {
                interval: item_state.interval_in_mins(),
                step: step + 1,
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            })
        } else {
            NextState::Review(ReviewState {
                interval_days: item_state.interval_in_days(),
                difficulty: item_state.difficulty,
                stability: item_state.stability,
            })
        }
    }

    fn easy(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.easy).into();

        NextState::Review(ReviewState {
            interval_days: item_state.interval_in_days(),
            difficulty: item_state.difficulty,
            stability: item_state.stability,
        })
    }
}
