#![forbid(unsafe_code)]
#![feature(associated_type_defaults)]
pub mod comptime;
pub mod datatype;
pub mod export;
pub mod runtime;
pub mod typegen;

pub use typegen::*;
pub use typegen_derive::Typegen;

pub mod internal {
    use std::borrow::Cow;

    #[derive(Clone, Debug)]
    pub enum Deprecation {
        /// A raw deprecated annotation with no additional information.
        ///
        /// `#[deprecated]`
        Deprecated,
        /// A deprecated annotation with a message and optional `since` version.
        ///
        /// `#[deprecated("Use `foobar()` instead")]
        /// `#[deprecated(since = "1.0.0", message = "Use `foobar` instead")]`
        DeprecatedWithMeta(DeprecationAttributes),
    }

    #[derive(Clone, Debug, Default)]
    pub struct DeprecationAttributes {
        pub message: Cow<'static, str>,
        pub since: Option<Cow<'static, str>>,
    }
}
