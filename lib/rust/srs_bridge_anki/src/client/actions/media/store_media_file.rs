use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum StoreMediaFileRequest {
    Data {
        filename: String,
        data: String,
        delete_existing: Option<bool>,
    },
    Path {
        filename: String,
        path: String,
        delete_existing: Option<bool>,
    },
    Url {
        filename: String,
        url: String,
        delete_existing: Option<bool>,
    },
}

impl AnkiRequest for StoreMediaFileRequest {
    type Response = String;

    const ACTION: &'static str = "storeMediaFile";
    const VERSION: u8 = 6;
}
