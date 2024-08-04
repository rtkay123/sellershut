#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations
)]

//! sellershut API utilities
#[cfg(feature = "telemetry")]
#[cfg_attr(docsrs, doc(cfg(feature = "telemetry")))]
/// Telemetry services
pub mod telemetry;

/// State
#[cfg_attr(docsrs, doc(cfg(any(feature = "meilisearch", feature = "postgres"))))]
pub mod state;

use thiserror::Error;

/// Errors returned by services
#[derive(Error, Debug)]
pub enum ServiceError {
    #[cfg(feature = "telemetry")]
    #[error(transparent)]
    /// When creating the tracing layer
    Loki(#[from] tracing_loki::Error),
    #[cfg(feature = "telemetry")]
    #[error(transparent)]
    /// When parsing url
    LokiUrl(#[from] tracing_loki::url::ParseError),
}
