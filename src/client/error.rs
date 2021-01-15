use std::fmt::{self, Display, Formatter};

pub use crate::api::HmcParseError;
pub use http::Error as HttpError;
pub use reqwest::Error as ReqwestError;
pub use tonic::{transport::Error as TransportError, Code, Status};

/// Result type used by many `Client` methods.
pub type ClientResult<T> = Result<T, ClientError>;

/// Error type used by `Client`.
#[derive(Debug)]
pub enum ClientError {
    /// Returned if an error occurs in the gRPC server or client.
    Grpc(Status),
    /// Returned if an error occurs on gRPC's transport layer.
    Transport(TransportError),
    /// Returned if an error occurs with the HTTP client.
    Reqwest(ReqwestError),
    /// Returned if an error occurs while creating HTTP requests / parsing for URLs.
    Http(HttpError),
    /// Returned if an authentication session isn't in progress, but authentication step methods were called.
    NoAuthId,
    /// Returned if the client is unauthenticated, but an API endpoint requires authentication.
    Unauthenticated,
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ClientError::Grpc(status) => write!(f, "An error occured in the gRPC server or client: {}", status),
            ClientError::Transport(transport_err) => write!(f, "And error occured in gRPC's transport layer: {}", transport_err),
            ClientError::Reqwest(reqwest_err) => write!(f, "An error occured in HTTP client, or request was unsuccessful: {}", reqwest_err),
            ClientError::Http(http_err) => write!(f, "An error occured while parsing an URL / creating an HTTP request: {}", http_err),
            ClientError::NoAuthId => write!(f, "No authentication session is in progress, but client tries to call auth API methods that need it"),
            ClientError::Unauthenticated => write!(f, "Client is not authenticated, but the API it tries to call requires authentication"),
        }
    }
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

impl From<ReqwestError> for ClientError {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}

impl From<HttpError> for ClientError {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}
