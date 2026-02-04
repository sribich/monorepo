use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::client::AnkiRequest;

/// Parameters for retrieving information about cards.
#[derive(Default, Debug, Clone, Serialize)]
pub struct CardsInfoRequest {
    /// The list of card IDs.
    pub cards: Vec<usize>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsInfoResponse {
    /// The answer side of the card.
    pub answer: String,
    /// The question side of the card.
    pub question: String,
    /// The name of the deck the card belongs to.
    pub deck_name: String,
    /// The name of the model (note type) of the card.
    pub model_name: String,
    /// The order of the fields.
    pub field_order: usize,
    /// The fields of the card.
    pub fields: HashMap<String, CardsInfoField>,
    /// The CSS style applied to the card.
    pub css: String,
    /// The ID of the card.
    pub card_id: usize,
    /// The interval of the card.
    pub interval: usize,
    /// The ID of the note that the card belongs to.
    pub note: usize,
    /// The ordinal value of the card.
    pub ord: usize,
    /// The type of the card.
    #[serde(rename = "type")]
    pub type_field: usize,
    /// The queue of the card.
    pub queue: usize,
    /// The due date of the card.
    pub due: isize,
    /// The number of repetitions of the card.
    pub reps: usize,
    /// The number of lapses of the card.
    pub lapses: usize,
    /// The number of cards left in the card's queue.
    pub left: usize,
    /// The modification time of the card.
    #[serde(rename = "mod")]
    pub r#mod: usize,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CardsInfoField {
    /// The value of the facade.
    pub value: String,
    /// The order of the facade.
    pub order: usize,
}

impl AnkiRequest for CardsInfoRequest {
    type Response = Vec<CardsInfoResponse>;

    const ACTION: &'static str = "cardsInfo";
    const VERSION: u8 = 6;
}
