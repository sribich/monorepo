use core::fmt::Debug;

use railgun_error::Error;
use railgun_error::Location;
use railgun_error::ResultExt;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json::json;

use crate::serde::SerializationError;
use crate::serde::to_path;

#[derive(Error)]
pub enum ForvoError {
    #[error(display("Reqwest error"))]
    Reqwest {
        error: reqwest::Error,
        location: Location,
    },
    #[error(display("Failed to serialize forvo request"))]
    Serialization {
        source: SerializationError,
        location: Location,
    },
}

pub type Result<T> = core::result::Result<T, ForvoError>;

pub trait ForvoRequest: Debug + Serialize {
    type Response: DeserializeOwned;

    const ACTION: &'static str;
    const FORMAT: &'static str;

    fn to_json(&self) -> Value {
        json!(self)
    }
}

pub struct ForvoClient {
    /// A Forvo API key.
    ///
    /// Your API key can be located at <https://api.forvo.com/account/>.
    api_key: String,
    /// A reusable reqwest client.
    client: Client,
}

impl ForvoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn request<R>(&self, data: R) -> Result<R::Response>
    where
        R: ForvoRequest + Send,
    {
        let api_key = &self.api_key;
        let format = R::FORMAT;
        let action = R::ACTION;

        let path = to_path(data).context(SerializationContext {})?;

        let url = format!(
            "https://apicommercial.forvo.com/key/{api_key}/format/{format}/action/{action}{path}"
        );

        self.client
            .get(url)
            .send()
            .await
            .context(ReqwestContext {})?
            .json::<R::Response>()
            .await
            .context(ReqwestContext {})
    }
}
