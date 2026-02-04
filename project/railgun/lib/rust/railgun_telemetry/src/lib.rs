pub mod settings;

#[cfg(feature = "logging")]
pub(crate) mod logging;

#[cfg(feature = "metrics")]
pub(crate) mod metrics;

#[cfg(feature = "tracing")]
pub(crate) mod tracing;

use std::sync::atomic::{AtomicBool, Ordering};

use railgun_core::{
    ServiceInfo,
    bootstrap::{BootstrapError, BootstrapResult, GenericContext, InitializationContext},
};
use railgun_error::ensure;
use settings::TelemetrySettings;

pub mod _internal_for_macros_ {
    #[cfg(feature = "settings")]
    pub use serde;
}

/// Tracks whether [`init`] has been called.
static TELEMETRY_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Returns `true` when [`init`] has been called.
fn is_initialized() -> bool {
    TELEMETRY_INITIALIZED.load(Ordering::Relaxed)
}

pub fn init(service_info: ServiceInfo, settings: &TelemetrySettings) -> Result<(), BootstrapError> {
    ensure!(
        !is_initialized(),
        GenericContext {
            reason: "telemetry::init should only be called once"
        },
    );

    // #[cfg(any(feature = "logging", feature = "metrics", feature = "tracing"))]
    // let _ = TelemetryContext::new(service_info);

    // Order of module initialisation matters here.
    #[cfg(feature = "tracing")]
    tracing::init::init(&service_info, settings);

    #[cfg(feature = "metrics")]
    metrics::init::init();

    #[cfg(feature = "logging")]
    logging::init::init();

    Ok(())
}
