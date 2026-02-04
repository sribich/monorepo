use std::time::Duration;

use opentelemetry::global::set_meter_provider;
use opentelemetry_otlp::{MetricExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    metrics::{PeriodicReader, PeriodicReaderBuilder, SdkMeterProvider},
};

pub(crate) fn init() -> SdkMeterProvider {
    let exporter = MetricExporter::builder()
        .with_http()
        // .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        // .with_endpoint("http://127.0.0.1:4318".to_owned())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(5))
        .build();

    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::builder().with_service_name("TODO").build())
        .build();

    set_meter_provider(provider.clone());

    provider
}
