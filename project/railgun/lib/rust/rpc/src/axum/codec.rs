use axum::body::Bytes;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Allows for encoding and decoding WebSocket messages.
pub trait Codec {
    type Error;

    fn encode<T>(msg: T) -> Result<Bytes, Self::Error>
    where
        T: Serialize;

    fn decode<T>(buf: Bytes) -> Result<T, Self::Error>
    where
        T: DeserializeOwned;
}

// #[derive(Debug)]
// #[non_exhaustive]
pub struct JsonCodec {}

impl Codec for JsonCodec {
    type Error = serde_json::Error;

    fn encode<T>(msg: T) -> Result<Bytes, Self::Error>
    where
        T: Serialize,
    {
        serde_json::to_vec(&msg).map(Into::into)
    }

    fn decode<T>(buf: Bytes) -> Result<T, Self::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(&buf)
    }
}
