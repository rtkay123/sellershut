use std::path::Path;

use config::{Config, Configuration};
use sqlx::{postgres::PgPoolOptions, PgPool};

mod config;

#[derive(Clone)]
pub struct ApiState {
    pub config: Config,
    pub db_pool: PgPool,
}

impl ApiState {
    pub async fn initialise() -> anyhow::Result<Self> {
        let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenvy::from_path(man_path).ok();

        let _log_handle = sellershut_services::telemetry::Handle::initialise();
        let config = Configuration::new();

        let pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, with 3 reserved for superusers.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(config.db_pool_max_size)
            .connect(&config.db_dsn)
            .await?;

        Ok(Self {
            config,
            db_pool: pool,
        })
    }
}
