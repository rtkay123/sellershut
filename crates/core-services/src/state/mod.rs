/// Config
pub mod config;

/// Service Error Types
pub mod error;

/// Events
pub mod events;

#[cfg(feature = "nats")]
use async_nats::jetstream::Context;

use config::Configuration;
use error::StateError;

#[cfg(feature = "postgres")]
use sqlx::{postgres::PgPoolOptions, PgPool};

#[cfg(feature = "cache")]
use crate::cache::RedisPool;

#[derive(Clone, Debug)]
/// Service state
pub struct ServiceState {
    /// App configuration
    pub config: std::sync::Arc<Configuration>,
    #[cfg(feature = "postgres")]
    /// Postgres connection pool
    pub db_pool: PgPool,
    #[cfg(feature = "cache")]
    /// Cache
    pub cache: RedisPool,
    #[cfg(feature = "nats")]
    /// Jetstream context
    pub jetstream_context: Context,
}

impl ServiceState {
    /// Initialise state
    pub async fn initialise(config: Configuration) -> Result<Self, StateError> {
        #[cfg(feature = "postgres")]
        let pool = {
            let pool = PgPoolOptions::new()
                // The default connection limit for a Postgres server is 100 connections, with 3 reserved for superusers.
                //
                // If you're deploying your application with multiple replicas, then the total
                // across all replicas should not exceed the Postgres connection limit.
                .max_connections(config.postgres.db_pool_max_size)
                .connect(&config.postgres.db_dsn);

            #[cfg(feature = "tracing")]
            let pool = tracing::Instrument::instrument(pool, tracing::info_span!("connect.db"));

            pool
        }
        .await?;

        #[cfg(feature = "nats")]
        let client = {
            let client = async_nats::connect(&config.nats.nats_url);

            #[cfg(feature = "tracing")]
            let client =
                tracing::Instrument::instrument(client, tracing::info_span!("connect.nats"));

            client
        }
        .await?;

        #[cfg(feature = "nats")]
        let jetstream = async_nats::jetstream::new(client);

        #[cfg(feature = "cache")]
        let cache = crate::cache::new_redis_pool_helper().await?;

        Ok(Self {
            config: std::sync::Arc::new(config),
            #[cfg(feature = "postgres")]
            db_pool: pool,
            #[cfg(feature = "cache")]
            cache,
            #[cfg(feature = "nats")]
            jetstream_context: jetstream,
        })
    }
}
