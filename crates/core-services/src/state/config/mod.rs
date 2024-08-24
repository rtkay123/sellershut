#[cfg(feature = "nats")]
mod nats;

#[cfg(feature = "nats")]
pub use nats::*;

#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "postgres")]
pub use postgres::*;

use std::{
    fmt::Display,
    net::{Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

pub(crate) type Config = Arc<Configuration>;

#[cfg_attr(feature = "postgres", derive(serde::Deserialize))]
/// Api Configuration
#[derive(Debug)]
pub struct Configuration {
    /// The environment in which to run the application.
    pub env: Environment,
    /// The address to listen on.
    pub listen_address: SocketAddr,
    /// The port to listen on.
    pub app_port: u16,
    /// Postgres Config
    #[cfg(feature = "postgres")]
    pub postgres: PostgresConfig,
    /// Query limit
    pub query_limit: i32,
    /// Loki URL
    pub loki_url: String,
    #[cfg(feature = "nats")]
    pub nats: NatsConfig,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "postgres", derive(serde::Deserialize))]
/// Runtime environment
pub enum Environment {
    /// Development
    Development,
    /// Production
    Production,
}

impl Configuration {
    /// Creates a new configuration from environment variables.
    pub fn new() -> Config {
        let env = env_var("APP_ENVIRONMENT")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENVIRONMENT environment variable. Please make sure it is either \"development\" or \"production\".");

        let app_port = env_var("PORT")
            .parse::<u16>()
            .expect("Unable to parse the value of the PORT environment variable. Please make sure it is a valid unsigned 16-bit integer");

        let query_limit = env_var("QUERY_LIMIT")
            .parse::<i32>()
            .expect("Max results to return per query");

        let loki_url = env_var("LOKI_URL");

        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        Arc::new(Configuration {
            env,
            listen_address,
            app_port,
            #[cfg(feature = "postgres")]
            postgres: {
                let db_pool_max_size = env_var("DATABASE_POOL_MAX_SIZE")
            .parse::<u32>()
            .expect("Unable to parse the value of the DATABASE_POOL_MAX_SIZE environment variable. Please make sure it is a valid unsigned 32-bit integer.");
                let db_dsn = env_var("DATABASE_URL");
                PostgresConfig {
                    db_pool_max_size,
                    db_dsn,
                }
            },
            query_limit,
            #[cfg(feature = "nats")]
            nats: {
                NatsConfig {
                    nats_url: env_var("NATS_URL"),
                    jetstream_name: env_var("JETSTREAM_NAME"),
                    jetstream_subjects: env_var("JETSTREAM_SUBJECTS")
                        .split('/')
                        .map(String::from)
                        .collect::<Vec<_>>(),
                    jetstream_max_bytes: env_var("JETSTREAM_MAX_BYTES")
                        .parse::<i64>()
                        .expect("JETSTREAM_MAX_BYTES to be i64"),
                }
            },
            loki_url,
        })
    }

    /// Sets the database DSN.
    /// This method is used in tests to override the database DSN.
    #[cfg(feature = "postgres")]
    pub fn set_dsn(&mut self, db_dsn: String) {
        self.postgres.db_dsn = db_dsn
    }
}

impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!(
                "Invalid environment: {}. Please make sure it is either \"development\" or \"production\".",
                s
            )),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Development => "development",
                Environment::Production => "production",
            }
        )
    }
}

/// Read environment variable
pub fn env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{}: {}", name, e))
        .expect("Missing environment variable")
}
