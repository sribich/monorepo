//!  - **cli**
//!
//!  - **settings**
//!
//!  - **security**
//!
//!  - **jemalloc**
//!  - **jemalloc-profiling**
//!
//!  - **telemetry**
//!  - **telemetry-client**
//!  - **telemetry-server**
//!
//!  - **metrics**
//!  - **logging**
//!  - **tracing**
#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "settings")]
pub use settings::*;

#[cfg(feature = "settings")]
mod settings;

/// Configure the global memory allocator to use [jemalloc].
///
/// The variable is `pub` solely for the purpose of documentation
/// and does not need to be imported to have an effect. As long as
/// the **jemalloc** feature is enabled, the allocator will be
/// configured automatically.
///
/// If no additional API is used from this crate, then the crate will
/// need to be explicitly linked into the target project by adding the
/// following snippet to either the `main.rs` or `lib.rs` entrypoint:
///
/// ```no_run
/// extern crate foundations;
/// ```
///
/// [jemalloc]: https://github.com/jemalloc/jemalloc
#[cfg(all(feature = "jemalloc", target_os = "linux"))]
#[global_allocator]
pub static JEMALLOC_MEMORY_ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[doc(hidden)]
pub mod _internal_for_macros_ {
    #[cfg(feature = "settings")]
    pub use serde;
}
