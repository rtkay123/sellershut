use std::error::Error;

pub mod mutation;
pub mod query;

pub fn map_err(err: impl Error) -> tonic::Status {
    tonic::Status::new(tonic::Code::Internal, err.to_string())
}
