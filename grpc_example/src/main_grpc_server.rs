use std::path::Path;

use config::{Config, File};
use log::*;
use opentelemetry::global::logger_provider;
use opentelemetry_appender_log::OpenTelemetryLogBridge;

use crate::telemetry::{
    logs_conf::init_logs,
    metrics_conf::init_metrics,
    traces_conf::{/*init_reqwest_tracing, */ init_reqwest_tracing, init_tracer},
};

mod domain;
mod grpc;
mod telemetry;

// Runtime to run our server
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //env_logger::init();

    info!("Read configuration ...");

    let settings: Config = Config::builder()
        .add_source(File::from(Path::new("./application.yml")))
        .build()
        .unwrap();

    let otlp_config = grpc::OtlpConfig {
        endpoint: settings
            .get_string("otel.endpoint")
            .unwrap_or("http://localhost:4317".to_string()),
    };

    info!(
        "OTel pipeline creating [Endpoint:{}]...",
        otlp_config.endpoint
    );

    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.
    let _ = init_reqwest_tracing(init_tracer(&otlp_config)?)?;
    let meter_provider = init_metrics(&otlp_config)?;
    // Initialize logs, which sets the global loggerprovider.
    let _ = init_logs(&otlp_config);

    // Retrieve the global LoggerProvider.
    let logger_provider = logger_provider();

    // Create a new OpenTelemetryLogBridge using the above LoggerProvider.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Info.to_level_filter());

    info!("OTel pipeline created");

    let addr = "[::1]:50051".parse()?;

    info!("listening on {}", addr);
    grpc::start_grpc_server(addr).await?;

    opentelemetry::global::shutdown_tracer_provider();
    opentelemetry::global::shutdown_logger_provider();
    meter_provider.shutdown()?;

    Ok(())
}
