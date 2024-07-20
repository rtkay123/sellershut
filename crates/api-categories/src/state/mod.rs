pub mod config;
mod impls;

use std::path::Path;

use config::{Config, Configuration};
use meilisearch_sdk::{client::Client, indexes::Index};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct ApiState {
    pub config: Config,
    pub db_pool: PgPool,
    pub meilisearch_index: Index,
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

        #[cfg(not(test))]
        sqlx::migrate!("./migrations").run(&pool).await?;

        let client = Client::new(&config.meilisearch_url, Some(&config.meilisearch_api_key))?;

        let index = client.index(&config.meilisearch_index);

        Ok(Self {
            config,
            db_pool: pool,
            meilisearch_index: index,
        })
    }
}
