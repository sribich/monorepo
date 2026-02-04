use reqwest::Client;
use tracing::error;

pub mod actions;
mod error;
mod request;

pub use error::*;
pub use request::*;

pub struct AnkiClient {
    endpoint: String,
    reqwest: Client,
}

impl AnkiClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            reqwest: Client::new(),
        }
    }

    pub async fn request<Request>(&self, params: Request) -> Result<Request::Response>
    where
        Request: AnkiRequest + Send,
    {
        let response = self
            .reqwest
            .post(&*self.endpoint)
            .json(&params.to_json())
            .send()
            .await
            .map_err(AnkiError::Reqwest)?
            .json::<AnkiResponse<Request::Response>>()
            .await
            .map_err(AnkiError::Reqwest)?;

        if let Some(error) = response.error {
            error!("{error:#?}");

            Err(AnkiError::Anki(error))
        } else if let Some(result) = response.result {
            Ok(result)
        } else {
            Ok(Default::default())
        }
    }
}

impl Default for AnkiClient {
    fn default() -> Self {
        Self::new("http://127.0.0.1:8765")
    }
}
