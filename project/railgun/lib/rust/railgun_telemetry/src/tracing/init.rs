use std::sync::OnceLock;

use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use railgun_core::ServiceInfo;

use crate::settings::TelemetrySettings;

fn get_resource(_service_info: &ServiceInfo) -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();

    RESOURCE
        .get_or_init(|| {
            Resource::builder()
                .with_service_name("basic-otlp-example-grpc")
                .build()
        })
        .clone()
}

pub(crate) fn init(service_info: &ServiceInfo, settings: &TelemetrySettings) {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        .with_endpoint(settings.endpoint.endpoint_url.clone())
        .build()
        .unwrap();

    // TODO: Trace config
    let _tracer = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource(service_info))
        .build();

    /*
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(settings.endpoint.endpoint_url.clone());
    let logger = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_resource(Resource::new([KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            service_info.identifier,
        )]))
        .with_exporter(exporter)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .unwrap();

    // Setup Log Appender for the log crate.
    let otel_log_appender =
        opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger);

    /////

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = Registry::default() //.with(layer); // .with(otel_log_appender);
        .with(env_filter)
        .with(layer)
        .with(otel_log_appender)
        .with(fmt::Layer::default());

    set_global_default(registry).unwrap();
     */
}

/*
use opentelemetry::{global::set_text_map_propagator, trace::Tracer, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, Resource};
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_log::LogTracer;
use tracing_opentelemetry::PreSampledTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

use super::error::TelemetryError;

    // let subscriber = get_subscriber("prelearning".into(), std::io::stdout);
    // init_subscriber(subscriber)?;

pub fn get_subscriber<Sink>(service_name: String, sink: Sink) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let tracing_layer = tracing_opentelemetry::layer()
        .with_tracer(init_tracing(service_name, "http://localhost:7281"));
    // Layer for printing spans to stdout
    // let formatting_layer = BunyanFormattingLayer::new(
    //     SERVICE_NAME.to_string(),
    //     std::io::stdout,
    // );

    set_text_map_propagator(TraceContextPropagator::new());

    /*
    subscriber
        .with(level_filter_layer)
        .with(tracing_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init()*/

    let format_layer = tracing_subscriber::fmt::layer()
        .with_writer(sink)
        .event_format(tracing_subscriber::fmt::format().pretty());

    Registry::default()
        .with(env_filter)
        .with(tracing_layer)
        .with(format_layer)
}



/// Register the provider subscriber as the global default to process span events.
///
/// # Errors
///
///  - When called more than once
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) -> Result<(), TelemetryError> {
    LogTracer::init().map_err(|prev| {
        TelemetryError::InitialisationError(
            "Failed to initialise the tracing-log redirector.",
            Box::new(prev),
        )
    })?;

    set_global_default(subscriber).map_err(|prev| {
        TelemetryError::InitialisationError(
            "Failed to set the default global subscriber.",
            Box::new(prev),
        )
    })?;

    Ok(())
}

/*
tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
*/

/*
// Create a gRPC exporter


    // Define a tracer


    // Define a subscriber.
    let subscriber = Registry::default();
    // Level filter layer to filter traces based on level (trace, debug, info, warn, error).
    let level_filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));
    // Layer for adding our configured tracer.
    let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // Layer for printing spans to stdout
    let formatting_layer = BunyanFormattingLayer::new(
        SERVICE_NAME.to_string(),
        std::io::stdout,
    );

    global::set_text_map_propagator(TraceContextPropagator::new());

    subscriber
        .with(level_filter_layer)
        .with(tracing_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init()
*/

*/

/*
use std::sync::Arc;

use opentelemetry::global::set_text_map_propagator;

use crate::BootstrapResult;

pub fn get_subscriber() {
    set_text_map_propagator(propagator)

    /*
        // Allows you to pass along context (i.e., trace IDs) across services
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    // Sets up the machinery needed to export data to Jaeger
    // There are other OTel crates that provide pipelines for the vendors
    // mentioned earlier.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("mini-redis")
        .install_simple()?;

    // Create a tracing layer with the configured tracer
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // The SubscriberExt and SubscriberInitExt traits are needed to extend the
    // Registry to accept `opentelemetry (the OpenTelemetryLayer type).
    tracing_subscriber::registry()
        .with(opentelemetry)
        // Continue logging to stdout
        .with(fmt::Layer::default())
        .try_init()?;
        */
}

pub fn init_tracing(// service_name: String,
                   // exporter_endpoint: &'static str,
) -> BootstrapResult<opentelemetry_sdk::trace::Tracer> /* -> impl Tracer + PreSampledTracer */
{
    // TODO: .with_endpoint(exporter_endpoint); <- from settings
    let exporter = opentelemetry_otlp::new_exporter().tonic();

    // TODO: Trace config
    opentelemetry_otlp::new_pipeline()
        .tracing()
        /*
        .with_trace_config(
            opentelemetry_sdk::trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                service_name,
            )])),
        )
        */
        .with_exporter(exporter)
        .install_batch(opentelemetry_sdk::runtime::Tokio)?
}
*/
