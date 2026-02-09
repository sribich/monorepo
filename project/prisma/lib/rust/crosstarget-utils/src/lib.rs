mod common;

#[cfg(not(target_arch = "wasm32"))]
mod native;
pub use crate::common::regex::RegExpCompat;
pub use crate::common::spawn::SpawnError;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::native::*;
