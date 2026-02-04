use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct RetrieveMediaFileRequest {
    pub filename: String,
}

impl AnkiRequest for RetrieveMediaFileRequest {
    type Response = String;

    const ACTION: &'static str = "retrieveMediaFile";
    const VERSION: u8 = 6;
}
