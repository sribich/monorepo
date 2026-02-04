#![feature(associated_type_defaults, type_alias_impl_trait)]
use std::{future::Future, pin::Pin};

pub mod di;
pub mod ext;

pub use rpc;
#[cfg(feature = "typegen")]
pub use typegen;

pub mod api {
    pub use railgun_api::*;
}

pub mod core {
    pub use railgun_core::*;
}

#[cfg(feature = "error")]
pub mod error {
    pub use railgun_error::*;
}

#[cfg(feature = "settings")]
pub mod settings {
    pub use railgun_settings::*;
}

#[cfg(feature = "telemetry")]
pub mod telemetry {
    pub use railgun_telemetry::*;
}

#[doc(hidden)]
pub mod _internal_for_macros_ {
    pub use async_trait::async_trait;
    #[cfg(feature = "settings")]
    pub use railgun_settings::_internal_for_macros_::*;
}
