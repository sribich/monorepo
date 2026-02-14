use std::fmt::Display;
use std::str::FromStr;

use shared::muid_new_newtype;
use shared::muid_newtype;

muid_newtype!(ResourceId);

pub enum ResourceState {
    Preparing,
    Committed,
}

impl FromStr for ResourceState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "preparing" => Ok(ResourceState::Preparing),
            "committed" => Ok(ResourceState::Committed),
            _ => Err(""),
        }
    }
}

impl Display for ResourceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceState::Preparing => write!(f, "preparing"),
            ResourceState::Committed => write!(f, "committed"),
        }
    }
}
