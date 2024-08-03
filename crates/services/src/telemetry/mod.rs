use std::collections::HashMap;

use tracing_loki::{url::Url, BackgroundTask};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::ServiceError;

/// Telemetry Handle
#[allow(missing_debug_implementations)]
pub struct Handle {
    /// A loki task
    pub loki_task: Option<BackgroundTask>,
}

/// Logging builder
#[allow(missing_debug_implementations)]
pub struct TelemetryBuilder {
    layer: tracing_subscriber::layer::Layered<
        tracing_subscriber::fmt::Layer<
            tracing_subscriber::layer::Layered<
                tracing_subscriber::EnvFilter,
                tracing_subscriber::Registry,
            >,
        >,
        tracing_subscriber::layer::Layered<
            tracing_subscriber::EnvFilter,
            tracing_subscriber::Registry,
        >,
    >,
    loki: Option<(tracing_loki::Layer, BackgroundTask)>,
}

#[derive(Clone, Copy, Debug)]
/// Configuration for Loki
pub struct LokiConfig<'a> {
    /// Labels
    pub labels: &'a HashMap<String, String>,
    /// Extra fields
    pub extra_fields: &'a HashMap<String, String>,
    /// Host
    pub host: &'a str,
}

impl TelemetryBuilder {
    /// Create the builder with a default log level if unset in environment
    pub fn new(default_level: &str) -> Self {
        let layer = tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| default_level.into()),
            )
            .with(tracing_subscriber::fmt::layer());

        Self { layer, loki: None }
    }

    /// Add Loki
    pub fn with_loki(self, config: &LokiConfig) -> Result<Self, ServiceError> {
        let mut builder = tracing_loki::builder();
        for (key, value) in config.labels.iter() {
            builder = builder.label(key, value)?;
        }

        for (key, value) in config.extra_fields.iter() {
            builder = builder.extra_field(key, value)?;
        }
        let url = builder.build_url(Url::parse(config.host)?)?;

        Ok(Self {
            layer: self.layer,
            loki: Some(url),
        })
    }

    /// Initialise logging
    pub fn build(self) -> Handle {
        match self.loki {
            Some((layer, task)) => {
                self.layer.with(layer).init();
                Handle {
                    loki_task: Some(task),
                }
            }
            None => {
                self.layer.init();
                Handle { loki_task: None }
            }
        }
    }
}
