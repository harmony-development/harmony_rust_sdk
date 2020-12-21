pub use tonic::{transport::Error as TransportError, Code, Status};

/// Result type used by many `Client` methods.
pub type ClientResult<T> = Result<T, ClientError>;

/// Error type used by `Client`.
#[derive(Debug)]
pub enum ClientError {
    Grpc(Status),
    Transport(TransportError),
}

impl From<Status> for ClientError {
    fn from(e: Status) -> Self {
        Self::Grpc(e)
    }
}

impl From<TransportError> for ClientError {
    fn from(e: TransportError) -> Self {
        Self::Transport(e)
    }
}
