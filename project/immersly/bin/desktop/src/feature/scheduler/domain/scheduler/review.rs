use super::CardScheduler;
use super::FsrsItemState;
use super::SchedulerState;
use crate::feature::scheduler::domain::card::Card;
use crate::feature::scheduler::domain::card::NextCardState;
use crate::feature::scheduler::domain::state::NextState;
use crate::feature::scheduler::domain::state::NextStates;
use crate::feature::scheduler::domain::state::RelearningState;
use crate::feature::scheduler::domain::state::ReviewState;

pub struct ReviewStateScheduler {
    state: SchedulerState,
}

impl ReviewStateScheduler {
    pub fn new(state: SchedulerState) -> Self {
        Self { state }
    }
}

impl CardScheduler for ReviewStateScheduler {
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
        todo!();
    }
}

impl ReviewStateScheduler {
    fn again(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.again).into();

        let interval = if let Some(mins) = state.relearning_steps.nth_as_mins(0) {
            mins
        } else {
            item_state.interval_in_mins()
        };

        NextState::Relearning(RelearningState {
            interval,
            step: 1,
            difficulty: item_state.difficulty,
            stability: item_state.stability,
        })
    }

    fn hard(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.hard).into();

        NextState::Review(ReviewState {
            interval_days: item_state.interval_in_days(),
            difficulty: item_state.difficulty,
            stability: item_state.stability,
        })
    }

    fn good(
        &self,
        card: &Card,
        state: &SchedulerState,
        fsrs_states: &fsrs::NextStates,
    ) -> NextState {
        let item_state: FsrsItemState = (&fsrs_states.good).into();

        NextState::Review(ReviewState {
            interval_days: item_state.interval_in_days(),
            difficulty: item_state.difficulty,
            stability: item_state.stability,
        })
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
