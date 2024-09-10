use std::path::Path;

use api_categories::state;
use core_services::{
    state::config::Configuration,
    tracing::{
        config::{AppMetadata, LokiConfig},
        TelemetryBuilder,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
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

    if let Some(task) = std::mem::take(&mut telemetry.loki_handle) {
        tokio::spawn(task);
    };

    let state = state::ApiState::initialise(config).await?;
    let (tx, _rx) = tokio::sync::oneshot::channel();

    tracing::error!("this is an error from tracing");

    api_categories::run(state, tx).await
}
