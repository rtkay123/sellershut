use thiserror::Error;

use crate::ServiceError;

#[derive(Error, Debug)]
/// State error
pub enum StateError {
    #[error(transparent)]
    /// Service error
    Service(#[from] ServiceError),
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    /// Postgres error
    Postgres(#[from] sqlx::Error),
    #[cfg(feature = "nats")]
    #[error(transparent)]
    /// Nats error
    Nats(#[from] async_nats::error::Error<async_nats::ConnectErrorKind>),
}
