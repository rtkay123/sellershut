use anyhow::Result;
use core_services::{
    state::config::Configuration,
    tracing::{
        config::{AppMetadata, LokiConfig},
        TelemetryBuilder,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let man_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenvy::from_path(man_path).ok();

    let crate_name = env!("CARGO_CRATE_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let config = Configuration::new(crate_name, crate_version);
    let metadata = AppMetadata {
        name: crate_name,
        version: crate_version,
        env: config.env,
    };

    let mut telemetry = TelemetryBuilder::new()
        .try_with_loki(LokiConfig::new(metadata, &config.loki_url))?
        .try_with_opentelemetry(metadata, &config.otel_collector)?
        .try_with_sentry(&config.sentry_dsn)?
        .build();

    if let Some(task) = std::mem::take(&mut telemetry.loki_task) {
        tokio::spawn(task);
    };

    api_search::serve(config).await
}
