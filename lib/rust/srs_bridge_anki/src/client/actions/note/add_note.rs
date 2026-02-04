use std::collections::HashMap;

use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct AddNoteRequest {
    pub note: Note,
}

#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub deck_name: String,
    pub model_name: String,
    pub fields: HashMap<String, String>,
    pub tags: Vec<String>,
}

impl AnkiRequest for AddNoteRequest {
    type Response = usize;

    const ACTION: &'static str = "addNote";
    const VERSION: u8 = 6;
}
