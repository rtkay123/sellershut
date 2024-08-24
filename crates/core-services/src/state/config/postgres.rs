use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// PostgresConfig
pub struct PostgresConfig {
    /// The DSN for the database. Currently, only PostgreSQL is supported.
    pub db_dsn: String,
    /// The maximum number of connections for the PostgreSQL pool.
    pub db_pool_max_size: u32,
}
