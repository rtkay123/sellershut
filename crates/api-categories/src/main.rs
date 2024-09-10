use std::path::Path;

use api_categories::state;
use core_services::{
    state::config::Configuration,
    tracing::{loki::LokiConfig, TelemetryBuilder},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenvy::from_path(man_path).ok();

    let crate_name = env!("CARGO_CRATE_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let config = Configuration::new(crate_name, crate_version);

    let mut telemetry = TelemetryBuilder::new()
        .with_env("info")
        .try_with_loki(LokiConfig::from(&config))?
        .build();

    if let Some(task) = std::mem::take(&mut telemetry.loki_handle) {
        tokio::spawn(task);
    };

    let state = state::ApiState::initialise(config).await?;
    let (tx, _rx) = tokio::sync::oneshot::channel();

    api_categories::run(state, tx).await
}
