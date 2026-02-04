use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeckNamesRequest;

impl AnkiRequest for DeckNamesRequest {
    type Response = Vec<String>;

    const ACTION: &'static str = "deckNames";
    const VERSION: u8 = 6;
}
