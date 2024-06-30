use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Telemetry Handle
#[derive(Debug)]
pub struct Handle {}

impl Handle {
    /// Initialise logging
    pub fn initialise(log_level: &str) -> Self {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| log_level.into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        Self {}
    }
}
