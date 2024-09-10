pub mod config;

#[cfg(feature = "tracing-loki")]
use config::LokiConfig;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

pub struct Telemetry {
    #[cfg(feature = "tracing-loki")]
    pub loki_handle: Option<tracing_loki::BackgroundTask>,
    #[cfg(feature = "sentry")]
    sentry_guard: Option<sentry::ClientInitGuard>,
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
    #[cfg(feature = "sentry")]
    sentry_guard: Option<sentry::ClientInitGuard>,
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
            #[cfg(feature = "sentry")]
            sentry_guard: None,
        }
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

    #[cfg(feature = "sentry")]
    pub fn try_with_sentry(mut self, dsn: &str) -> Result<Self, crate::ServiceError> {
        use sentry::{ClientOptions, IntoDsn};
        println!("dsn={dsn}");

        let guard = sentry::init(ClientOptions {
            dsn: dsn.into_dsn()?,
            traces_sample_rate: 1.0,
            release: sentry::release_name!(),
            ..Default::default()
        });

        self.layer
            .push(sentry::integrations::tracing::layer().boxed());
        self.sentry_guard = Some(guard);
        Ok(self)
    }

    #[cfg(feature = "opentelemetry")]
    pub fn try_with_opentelemetry(
        mut self,
        config: config::AppMetadata,
        endpoint: &str,
    ) -> Result<Self, crate::ServiceError> {
        use opentelemetry::{global, trace::TracerProvider, KeyValue};
        use opentelemetry_otlp::WithExportConfig;
        use opentelemetry_sdk::{
            runtime,
            trace::{BatchConfig, RandomIdGenerator, Sampler},
            Resource,
        };
        use opentelemetry_semantic_conventions::{
            resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
            SCHEMA_URL,
        };
        use tracing_opentelemetry::OpenTelemetryLayer;

        global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let resource = Resource::from_schema_url(
            [
                KeyValue::new(SERVICE_NAME, config.name.to_owned()),
                KeyValue::new(SERVICE_VERSION, config.version.to_owned()),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, config.env.to_string()),
            ],
            SCHEMA_URL,
        );

        let provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                opentelemetry_sdk::trace::Config::default()
                    .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                        1.0,
                    ))))
                    .with_id_generator(RandomIdGenerator::default())
                    .with_resource(resource),
            )
            .with_batch_config(BatchConfig::default())
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(endpoint),
            )
            .install_batch(runtime::Tokio)
            .unwrap();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer(config.name.to_string());

        self.layer.push(OpenTelemetryLayer::new(tracer).boxed());

        Ok(self)
    }

    pub fn build(self) -> Telemetry {
        tracing_subscriber::registry()
            .with(self.layer)
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
            .init();
        Telemetry {
            #[cfg(feature = "tracing-loki")]
            loki_handle: self.loki_handle,
            #[cfg(feature = "sentry")]
            sentry_guard: self.sentry_guard,
        }
    }
}
