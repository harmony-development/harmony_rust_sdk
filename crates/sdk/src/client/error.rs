use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

use hrpc::decode::DecodeBodyError;
use prost::DecodeError;

pub use crate::api::HmcParseError;
pub use hrpc::client::socket::SocketError;
pub use http::uri::InvalidUri as UrlError;
pub use reqwest::Error as ReqwestError;

/// Result type used by all `Client` methods.
pub type ClientResult<T> = Result<T, ClientError>;
/// Alias for an internal hRPC client error.
#[cfg(feature = "client_web")]
pub type InternalClientError =
    hrpc::client::error::ClientError<hrpc::client::transport::http::WasmError>;
/// Alias for an internal hRPC client error.
#[cfg(all(feature = "client_native", not(feature = "client_web")))]
pub type InternalClientError =
    hrpc::client::error::ClientError<hrpc::client::transport::http::HyperError>;

/// Error type used by `Client`.
#[derive(Debug)]
pub enum ClientError {
    /// Returned if the internal hRPC client returns an error.
    Internal(InternalClientError),
    /// Returned if an error occurs with the HTTP client.
    Reqwest(ReqwestError),
    /// Returned if an error occurs while creating HTTP requests / parsing for URLs.
    UrlParse(UrlError),
    /// Returned if an authentication session isn't in progress, but authentication step methods were called.
    NoAuthId,
    /// Returned if the client is unauthenticated, but an API endpoint requires authentication.
    Unauthenticated,
    /// Returned if a response from the server has invalid / empty value(s) according to the protocol.
    UnexpectedResponse(String),
    /// Returned if a socket returns an error.
    SocketError(SocketError),
}

impl ClientError {
    pub(crate) fn unexpected(msg: impl ToString) -> Self {
        ClientError::UnexpectedResponse(msg.to_string())
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ClientError::Internal(err) => write!(f, "An internal error occured: {}", err),
            ClientError::Reqwest(reqwest_err) => write!(f, "An error occured in HTTP client, or request was unsuccessful: {}", reqwest_err),
            ClientError::UrlParse(err) => write!(f, "An error occured while parsing an URL: {}", err),
            ClientError::NoAuthId => write!(f, "No authentication session is in progress, but client tries to call auth API methods that need it"),
            ClientError::Unauthenticated => write!(f, "Client is not authenticated, but the API it tries to call requires authentication"),
            ClientError::UnexpectedResponse(msg) => write!(f, "Server responded with unexpected value: {}", msg),
            ClientError::SocketError(err) => write!(f, "socket error: {}", err),
        }
    }
}

impl From<ReqwestError> for ClientError {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}

impl From<UrlError> for ClientError {
    fn from(e: UrlError) -> Self {
        Self::UrlParse(e)
    }
}

impl From<InternalClientError> for ClientError {
    fn from(e: InternalClientError) -> Self {
        Self::Internal(e)
    }
}

impl From<DecodeError> for ClientError {
    fn from(e: DecodeError) -> Self {
        Self::Internal(InternalClientError::MessageDecode(
            DecodeBodyError::InvalidProtoMessage(e),
        ))
    }
}

impl From<DecodeBodyError> for ClientError {
    fn from(e: DecodeBodyError) -> Self {
        Self::Internal(InternalClientError::MessageDecode(e))
    }
}

impl From<SocketError> for ClientError {
    fn from(e: SocketError) -> Self {
        Self::SocketError(e)
    }
}

impl StdError for ClientError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ClientError::Internal(err) => Some(err),
            ClientError::Reqwest(err) => Some(err),
            ClientError::UrlParse(err) => Some(err),
            _ => None,
        }
    }
}
