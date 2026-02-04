use std::collections::HashMap;

use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeckNamesAndIdsRequest;

impl AnkiRequest for DeckNamesAndIdsRequest {
    type Response = HashMap<String, usize>;

    const ACTION: &'static str = "deckNamesAndIds";
    const VERSION: u8 = 6;
}
