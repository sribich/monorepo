use std::collections::HashMap;

use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDecksRequest {
    pub cards: Vec<usize>,
}

impl AnkiRequest for GetDecksRequest {
    type Response = HashMap<String, Vec<usize>>;

    const ACTION: &'static str = "getDecks";
    const VERSION: u8 = 6;
}
