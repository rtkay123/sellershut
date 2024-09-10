pub mod loki;

#[cfg(feature = "tracing-loki")]
use loki::LokiConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

pub struct Telemetry {
    #[cfg(feature = "tracing-loki")]
    pub loki_handle: Option<tracing_loki::BackgroundTask>,
}

impl Telemetry {
    pub fn builder() -> TelemetryBuilder {
        TelemetryBuilder::default()
    }
}

pub struct TelemetryBuilder {
    layer: Vec<Box<dyn Layer<Registry> + Sync + Send>>,
    #[cfg(feature = "tracing-loki")]
    loki_handle: Option<tracing_loki::BackgroundTask>,
}

impl Default for TelemetryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryBuilder {
    pub fn new() -> Self {
        let types: Box<dyn Layer<Registry> + Sync + Send> =
            tracing_subscriber::fmt::layer().boxed();
        TelemetryBuilder {
            layer: vec![types],
            #[cfg(feature = "tracing-loki")]
            loki_handle: None,
        }
    }

    pub fn with_env(mut self, default_level: &str) -> Self {
        let layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| default_level.into())
            .boxed();
        self.layer.push(layer);
        self
    }

    #[cfg(feature = "tracing-loki")]
    pub fn try_with_loki(mut self, config: LokiConfig) -> Result<Self, crate::ServiceError> {
        let mut builder = tracing_loki::builder();
        for (key, value) in config.labels.iter() {
            builder = builder.label(key, value)?;
        }

        for (key, value) in config.extra_fields.iter() {
            builder = builder.extra_field(key, value)?;
        }
        let (layer, background_task) =
            builder.build_url(tracing_loki::url::Url::parse(&config.host)?)?;

        self.loki_handle = Some(background_task);

        self.layer.push(layer.boxed());
        Ok(self)
    }

    pub fn build(self) -> Telemetry {
        tracing_subscriber::registry().with(self.layer).init();
        Telemetry {
            #[cfg(feature = "tracing-loki")]
            loki_handle: self.loki_handle,
        }
    }
}
