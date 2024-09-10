#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations
)]

//! sellershut API utilities
#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
/// Tracing services
pub mod tracing;

/// State
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "meilisearch", feature = "postgres", feature = "cache")))
)]
pub mod state;

/// State
#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
#[cfg(feature = "cache")]
pub mod cache;

use thiserror::Error;

/// Errors returned by services
#[derive(Error, Debug)]
pub enum ServiceError {
    #[cfg(feature = "tracing-loki")]
    #[error(transparent)]
    /// When creating the tracing layer
    Loki(#[from] tracing_loki::Error),
    #[cfg(feature = "tracing-loki")]
    #[error(transparent)]
    /// When parsing url
    LokiUrl(#[from] tracing_loki::url::ParseError),
    #[cfg(feature = "cache")]
    #[error(transparent)]
    /// When creating the tracing layer
    Cache(#[from] redis::RedisError),
    #[cfg(feature = "opentelemetry")]
    #[error(transparent)]
    /// When creating the tracing layer
    Opentelemetry(#[from] opentelemetry::trace::TraceError),
    #[cfg(feature = "sentry")]
    #[error(transparent)]
    /// When creating the tracing layer
    SentryDsn(#[from] sentry::types::ParseDsnError),
}
