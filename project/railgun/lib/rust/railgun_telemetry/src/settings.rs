use railgun_settings::Settings;
use railgun_settings_derive::settings;

#[cfg_attr(feature = "settings", settings)]
pub struct TelemetrySettings {
    #[cfg(any(feature = "tracing", feature = "metrics", feature = "logging"))]
    pub endpoint: EndpointSettings,
    #[cfg(feature = "tracing")]
    pub tracing: TracingSettings,
    #[cfg(feature = "metrics")]
    pub metrics: MetricsSettings,
    #[cfg(feature = "logging")]
    pub logging: LoggingSettings,
}

#[cfg_attr(feature = "settings", settings)]
pub struct EndpointSettings {
    #[serde(default = "EndpointSettings::default_endpoint_url")]
    pub endpoint_url: String,
}

impl EndpointSettings {
    pub fn default_endpoint_url() -> String {
        "http://127.0.0.1:4317".to_owned()
    }
}

#[cfg_attr(feature = "settings", settings)]
pub struct TracingSettings {}

#[cfg_attr(feature = "settings", settings)]
pub struct MetricsSettings {}

#[cfg_attr(feature = "settings", settings)]
pub struct LoggingSettings {}
