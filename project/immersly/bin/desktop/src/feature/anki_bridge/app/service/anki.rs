use srs_bridge_anki::client::AnkiClient;

pub struct AnkiService {
    client: AnkiClient,
}

impl Default for AnkiService {
    fn default() -> Self {
        Self::new()
    }
}

impl AnkiService {
    pub fn new() -> Self {
        Self {
            client: AnkiClient::default(),
        }
    }

    pub fn client(&self) -> &AnkiClient {
        &self.client
    }
}
