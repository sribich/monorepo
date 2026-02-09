pub mod bootstrap;

pub mod _internal_for_macros_ {
    pub use const_str::convert_ascii_case;
    pub use const_str::replace;
}

#[derive(Clone, Copy, Debug)]
pub struct ServiceInfo {
    /// The name of the service
    pub name: &'static str,

    /// A version of the name which can be used as an identifier for
    /// additional services, such as metrics.
    ///
    /// The identifier is the name with the following modifications:
    ///
    ///   - It is lowercased
    ///   - Hyphens (-) are replaced with underscores (_)
    ///   - Spaces ( ) are replaced with underscores (_)
    pub identifier: &'static str,

    /// The version of the service.
    pub version: &'static str,

    /// The author of the service.
    pub author: &'static str,

    /// The description of the service.
    pub description: &'static str,
}

/// Creates a [`ServiceInfo`] struct from the metadata contained within the
/// `Cargo.toml` manifest of the service.
#[macro_export]
macro_rules! service_info {
    () => {{
        use $crate::_internal_for_macros_::convert_ascii_case;
        use $crate::_internal_for_macros_::replace;

        $crate::ServiceInfo {
            name: env!("CARGO_PKG_NAME"),
            identifier: replace!(
                replace!(convert_ascii_case!(lower, env!("CARGO_PKG_NAME")), "-", "_"),
                " ",
                "_"
            ),
            version: env!("CARGO_PKG_VERSION"),
            author: env!("CARGO_PKG_AUTHORS"),
            description: env!("CARGO_PKG_DESCRIPTION"),
        }
    }};
}
