use std::fmt::Display;
use std::str::FromStr;

use railgun::error::Error;
use railgun::error::Location;
use railgun::error::ResultExt;
use serde::Deserialize;
use serde::Serialize;
use typegen::Typegen;
use uuid::Uuid;

#[derive(Error)]
pub enum MuidParseError {
    InvalidByteLength {
        error: uuid::Error,
        location: Location,
    },
    InvalidUuid {
        error: uuid::Error,
        location: Location,
    },
}

/// A `Muid` is our internal representation of IDs. They are a thinly
/// veiled UUIDv7 with checks to ensure that they remain in a valid
/// state.
///
/// TODO: This should not be Typegen/Serialize/Deserialize.
#[derive(Debug, Clone, Typegen, Serialize, Deserialize)]
pub struct Muid(Uuid);

impl Muid {
    pub fn new_now() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    pub fn try_from_slice(slice: &[u8]) -> Result<Self, MuidParseError> {
        Ok(Self(
            Uuid::from_slice(slice).context(InvalidByteLengthContext {})?,
        ))
    }

    pub fn try_from_str<S: AsRef<str>>(data: S) -> Result<Self, MuidParseError> {
        Ok(Self(
            Uuid::from_str(data.as_ref()).context(InvalidUuidContext {})?,
        ))
    }

    pub fn from_slice_unchecked(slice: &[u8]) -> Self {
        Self(Uuid::from_slice(slice).unwrap())
    }
}

impl Display for Muid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.simple())
    }
}
