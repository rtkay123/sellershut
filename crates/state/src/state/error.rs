use sellershut_services::ServiceError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateError {
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[cfg(feature = "meilisearch")]
    #[error(transparent)]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    Postgres(#[from] sqlx::Error),
}
