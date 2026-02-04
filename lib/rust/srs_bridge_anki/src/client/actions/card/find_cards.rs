use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct FindCardsRequest {
    pub query: String,
}

impl AnkiRequest for FindCardsRequest {
    type Response = Vec<usize>;

    const ACTION: &'static str = "findCards";
    const VERSION: u8 = 6;
}
