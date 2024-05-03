use crate::grpc::OtlpConfig;
use opentelemetry::logs::LogError;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::logs::Config;
use opentelemetry_sdk::runtime;

use super::traces_conf::get_resource_attr;

pub fn init_logs(otlp_config: &OtlpConfig) -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(Config::default().with_resource(get_resource_attr()))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_config.endpoint.clone())
                .with_protocol(Protocol::Grpc),
        )
        .install_batch(runtime::Tokio)
}
