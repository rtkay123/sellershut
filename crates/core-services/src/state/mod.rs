/// Config
pub mod config;

/// Service Error Types
pub mod error;

#[cfg(feature = "nats")]
use async_nats::jetstream::stream::Config;

use config::Configuration;
use error::StateError;

#[cfg(feature = "postgres")]
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone, Debug)]
/// Service state
pub struct ServiceState {
    /// App configuration
    pub config: std::sync::Arc<Configuration>,
    #[cfg(feature = "postgres")]
    /// Postgres connection pool
    pub db_pool: PgPool,
}

impl ServiceState {
    /// Initialise state
    pub async fn initialise(_crate_name: &str) -> Result<Self, StateError> {
        let config = Configuration::new();

        #[cfg(all(feature = "rt-tokio", feature = "telemetry"))]
        {
            use crate::telemetry::*;
            use std::{collections::HashMap, process};

            let mut labels = HashMap::new();
            labels.insert("environment".into(), config.env.to_string());
            labels.insert("application".into(), _crate_name.to_string());

            let mut extra_fields = HashMap::new();
            extra_fields.insert("pid".into(), format!("{}", process::id()));
            let loki_config = LokiConfig {
                labels: &labels,
                extra_fields: &extra_fields,
                host: &config.loki_url,
            };
            let mut log_handle = TelemetryBuilder::new("info")
                .with_loki(&loki_config)?
                .build();

            if let Some(task) = std::mem::take(&mut log_handle.loki_task) {
                tokio::spawn(task);
            };
        }

        #[cfg(feature = "postgres")]
        let pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, with 3 reserved for superusers.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(config.postgres.db_pool_max_size)
            .connect(&config.postgres.db_dsn)
            .await?;

        #[cfg(feature = "nats")]
        let client = async_nats::connect(&config.nats.nats_url).await.unwrap();
        #[cfg(feature = "nats")]
        let jetstream = async_nats::jetstream::new(client);

        #[cfg(feature = "nats")]
        jetstream
            .get_or_create_stream(Config {
                name: config.nats.jetstream_name.to_string(),
                subjects: config.nats.jetstream_subjects.clone(),
                max_bytes: config.nats.jetstream_max_bytes,
                ..Default::default()
            })
            .await
            .unwrap();

        Ok(Self {
            config,
            #[cfg(feature = "postgres")]
            db_pool: pool,
        })
    }
}
