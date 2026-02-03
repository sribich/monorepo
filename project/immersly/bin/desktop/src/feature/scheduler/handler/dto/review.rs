use railgun::typegen::Typegen;
use serde::Serialize;

use crate::feature::scheduler::domain::card::Card;
use crate::feature::scheduler::domain::state::NextStates;

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub card: Card,
    pub next_states: NextStates,
}
