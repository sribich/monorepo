use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;

use super::card_state::CardState;

#[derive(Debug, Typegen, Serialize, Deserialize)]
pub struct NextCardState {
    pub state: CardState,
    pub due: u64,
    pub stability: Option<f64>,
    pub difficulty: Option<f64>,
}
