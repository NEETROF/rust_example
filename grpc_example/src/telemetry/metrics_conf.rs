use opentelemetry::metrics::{self};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{metrics::SdkMeterProvider, runtime};

use crate::grpc::OtlpConfig;

use super::traces_conf::get_resource_attr;

pub fn init_metrics(otlp_config: &OtlpConfig) -> metrics::Result<SdkMeterProvider> {
    let export_config = ExportConfig {
        endpoint: otlp_config.endpoint.clone(),
        ..ExportConfig::default()
    };
    let provider = opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(get_resource_attr())
        .build();

    provider
}
