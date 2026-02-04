use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json::json;

#[derive(Deserialize)]
pub struct AnkiResponse<T> {
    pub result: Option<T>,
    pub error: Option<String>,
}

pub trait AnkiRequest: Debug + Serialize {
    type Response: Default + DeserializeOwned;

    const ACTION: &'static str;
    const VERSION: u8;

    fn to_json(&self) -> Value {
        if json!(self).is_null() {
            json!({
                "action": Self::ACTION,
                "version": Self::VERSION,
            })
        } else {
            json!({
                "action": Self::ACTION,
                "version": Self::VERSION,
                "params": self
            })
        }
    }
}
