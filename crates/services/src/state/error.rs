use thiserror::Error;

use crate::ServiceError;

#[derive(Error, Debug)]
/// State error
pub enum StateError {
    #[error(transparent)]
    /// Service error
    Service(#[from] ServiceError),
    #[cfg(feature = "meilisearch")]
    #[error(transparent)]
    /// Meilisearch error
    Meilisearch(#[from] meilisearch_sdk::errors::Error),
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    /// Postgres error
    Postgres(#[from] sqlx::Error),
}
