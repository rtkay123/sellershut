// #[cfg(feature = "nats")]
// mod nats;
//
// #[cfg(feature = "nats")]
// #[cfg_attr(docsrs, doc(cfg(feature = "nats")))]
// pub use nats::*;

#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub use postgres::*;

#[cfg(feature = "api")]
use std::net::{Ipv6Addr, SocketAddr};
use std::{fmt::Display, str::FromStr};

#[cfg_attr(feature = "postgres", derive(serde::Deserialize))]
/// Api Configuration
#[derive(Debug)]
pub struct Configuration {
    /// The environment in which to run the application.
    pub env: Environment,
    /// The address to listen on.
    #[cfg(feature = "api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api")))]
    pub listen_address: SocketAddr,
    /// The port to listen on.
    #[cfg(feature = "api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api")))]
    pub app_port: u16,
    /// Postgres Config
    #[cfg(feature = "postgres")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
    pub postgres: PostgresConfig,
    /// Query limit
    #[cfg(feature = "api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api")))]
    pub query_limit: i32,
    /// Loki URL
    #[cfg(feature = "tracing-loki")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tracing-loki")))]
    pub loki_url: String,
    #[cfg(feature = "nats")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nats")))]
    /// Nats Url
    pub nats_url: String,
    /// Pkg name
    pub pkg_name: String,
    /// Package version
    pub pkg_version: String,
    /// Package version
    #[cfg(feature = "opentelemetry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry")))]
    pub otel_collector: String,
    #[cfg(feature = "sentry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sentry")))]
    /// Sentry dsn
    pub sentry_dsn: String,
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
    pub fn new(crate_name: &str, crate_version: &str) -> Self {
        let env = env_var("APP_ENVIRONMENT")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENVIRONMENT environment variable. Please make sure it is either \"development\" or \"production\".");

        #[cfg(feature = "api")]
        let app_port = env_var("PORT")
            .parse::<u16>()
            .expect("Unable to parse the value of the PORT environment variable. Please make sure it is a valid unsigned 16-bit integer");

        #[cfg(feature = "api")]
        let query_limit = env_var("QUERY_LIMIT")
            .parse::<i32>()
            .expect("Max results to return per query");

        #[cfg(feature = "tracing-loki")]
        let loki_url = env_var("LOKI_URL");

        #[cfg(feature = "opentelemetry")]
        let otel_collector = env_var("OPENTELEMETRY_COLLECTOR_HOST");

        #[cfg(feature = "sentry")]
        let sentry_dsn = env_var("SENTRY_DSN");

        #[cfg(feature = "api")]
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        Configuration {
            env,
            #[cfg(feature = "api")]
            listen_address,
            #[cfg(feature = "api")]
            app_port,
            #[cfg(feature = "nats")]
            nats_url: env_var("NATS_URL"),
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
            #[cfg(feature = "api")]
            query_limit,
            #[cfg(feature = "tracing-loki")]
            loki_url,
            pkg_name: crate_name.to_string(),
            pkg_version: crate_version.to_string(),
            #[cfg(feature = "opentelemetry")]
            otel_collector,
            #[cfg(feature = "sentry")]
            sentry_dsn,
        }
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
