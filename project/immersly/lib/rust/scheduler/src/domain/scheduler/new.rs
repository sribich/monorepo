use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use super::CardScheduler;
use crate::domain::entity::card::Card;
use crate::domain::state::NextStates;
use crate::domain::value::card_state::CardState;
use crate::domain::value::next_card_state::NextCardState;

pub struct NewCardScheduler {}

impl CardScheduler for NewCardScheduler {
    fn get_next_states(&self, card: &Card) -> NextStates {
        // let current_step = car
        todo!();
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
