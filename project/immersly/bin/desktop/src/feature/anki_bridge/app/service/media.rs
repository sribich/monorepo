use std::sync::Arc;

use railgun_di::Component;
use srs_bridge_anki::client::actions::media::store_media_file::StoreMediaFileRequest;

use super::anki::AnkiService;

#[derive(Component)]
pub struct AnkiMediaService {
    client: Arc<AnkiService>,
}

impl AnkiMediaService {
    pub async fn add_media_file(&self, filename: String, path: String) -> String {
        self.client
            .client()
            .request(StoreMediaFileRequest::Path {
                filename,
                path,
                delete_existing: Some(true),
            })
            .await
            .unwrap()
    }
}
