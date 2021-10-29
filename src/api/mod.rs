use std::{
    convert::TryFrom,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use hrpc::exports::http::{uri::InvalidUri as UrlParseError, Uri};
use http::uri::{Parts, Scheme};

/// Rest API common types.
pub mod rest;

/// Chat service API.
#[cfg(feature = "gen_chat")]
pub mod chat;

/// Auth service API.
#[cfg(feature = "gen_auth")]
pub mod auth {
    pub mod v1 {
        #![allow(clippy::unit_arg)]
        hrpc::include_proto!("protocol.auth.v1");
    }
    pub use v1::*;
}

/// Common types used in other services.
#[cfg(feature = "gen_harmonytypes")]
pub mod harmonytypes;

/// Media proxy service API.
#[cfg(feature = "gen_mediaproxy")]
pub mod mediaproxy {
    pub mod v1 {
        hrpc::include_proto!("protocol.mediaproxy.v1");
    }
    pub use v1::*;
}

/// Voice service API.
#[cfg(feature = "gen_voice")]
pub mod voice {
    pub mod v1 {
        hrpc::include_proto!("protocol.voice.v1");
    }
    pub use v1::*;
}

/// Sync service API.
#[cfg(feature = "gen_sync")]
pub mod sync {
    pub mod v1 {
        hrpc::include_proto!("protocol.sync.v1");
    }
    pub use v1::*;
}

/// Profile service API.
#[cfg(feature = "gen_profile")]
pub mod profile;

/// Emote service API.
#[cfg(feature = "gen_emote")]
pub mod emote {
    pub mod v1 {
        hrpc::include_proto!("protocol.emote.v1");
    }
    pub use v1::*;
}
/// Batch service API.
#[cfg(feature = "gen_batch")]
pub mod batch {
    pub mod v1 {
        hrpc::include_proto!("protocol.batch.v1");
    }
    pub use v1::*;
}

/// Some crates re-exported for user convenience.
pub mod exports {
    pub use hrpc;
    pub use prost;
}

/// Errors that can occur when converting a possibly non-HMC URL to a HMC URL.
#[derive(Debug, Clone)]
pub enum HmcParseError {
    /// Returned when no server could be extracted.
    NoServer,
    /// Returned when no ID could be extracted.
    NoId,
    /// Returned if the ID is invalid.
    ///
    /// Currently only returned if the URL path contains '/'.
    InvalidId,
}

impl Display for HmcParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HmcParseError::NoServer => write!(f, "Missing a server part in URL"),
            HmcParseError::NoId => write!(f, "Missing an ID part in URL"),
            HmcParseError::InvalidId => write!(f, "Invalid ID in URL"),
        }
    }
}

impl StdError for HmcParseError {}

/// Errors that can occur when converting a possibly non-HMC string to a HMC URL.
#[derive(Debug)]
pub enum HmcFromStrError {
    /// Returned if the string isn't a valid URL.
    UrlParse(UrlParseError),
    /// Returned if the string isn't a valid HMC URL.
    HmcParse(HmcParseError),
}

impl Display for HmcFromStrError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HmcFromStrError::HmcParse(err) => write!(f, "{}", err),
            HmcFromStrError::UrlParse(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for HmcFromStrError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            HmcFromStrError::HmcParse(err) => err,
            HmcFromStrError::UrlParse(err) => err,
        })
    }
}

/// A HMC.
///
/// An example HMC looks like `hmc://chat.harmonyapp.io/403cb46c-49cf-4ae1-b876-f38eb26accb0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hmc {
    inner: Uri,
}

impl Hmc {
    /// Validates and creates a new HMC given a homeserver and (attachment) ID.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::Hmc;
    /// let hmc = Hmc::new("example.org", "403cb46c-49cf-4ae1-b876-f38eb26accb0").unwrap();
    /// assert_eq!(hmc.to_string(), "hmc://example.org/403cb46c-49cf-4ae1-b876-f38eb26accb0");
    /// ```
    pub fn new(
        server: impl std::fmt::Display,
        id: impl std::fmt::Display,
    ) -> Result<Self, HmcFromStrError> {
        Self::from_str(&format!("hmc://{}/{}", server, id))
    }

    /// Gets the ID of this HMC.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::Hmc;
    /// let hmc = Hmc::new("example.org", "403cb46c-49cf-4ae1-b876-f38eb26accb0").unwrap();
    /// assert_eq!(hmc.id(), "403cb46c-49cf-4ae1-b876-f38eb26accb0");
    /// ```
    pub fn id(&self) -> &str {
        self.inner.path().trim_start_matches('/')
    }

    /// Gets the server of this HMC.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::Hmc;
    /// let hmc = Hmc::new("example.org", "403cb46c-49cf-4ae1-b876-f38eb26accb0").unwrap();
    /// assert_eq!(hmc.server(), "example.org");
    /// ```
    pub fn server(&self) -> &str {
        self.inner.host().unwrap()
    }

    /// Gets the port of this HMC.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::Hmc;
    /// let hmc = Hmc::new("example.org:2289", "403cb46c-49cf-4ae1-b876-f38eb26accb0").unwrap();
    /// assert_eq!(hmc.port(), 2289);
    /// ```
    pub fn port(&self) -> u16 {
        self.inner.port_u16().unwrap_or(2289)
    }
}

impl Display for Hmc {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<Hmc> for String {
    fn from(hmc: Hmc) -> String {
        hmc.inner.to_string()
    }
}

impl FromStr for Hmc {
    type Err = HmcFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = value.parse::<Uri>().map_err(HmcFromStrError::UrlParse)?;

        Self::try_from(url).map_err(HmcFromStrError::HmcParse)
    }
}

impl TryFrom<Uri> for Hmc {
    type Error = HmcParseError;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        if value.host().is_none() {
            return Err(HmcParseError::NoServer);
        };

        // We trim the first '/' if it exists since it will always be the authority - path seperator
        let path = value.path().trim_start_matches('/');
        if path.is_empty() {
            return Err(HmcParseError::NoId);
        } else if path.contains('/') {
            return Err(HmcParseError::InvalidId);
        }

        Ok(Self { inner: value })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_HMC: &str = "hmc://chat.harmonyapp.io:2289/fdeded13-844b-42e1-b813-34f74f9afdbc";
    const INVALID_ID_HMC: &str =
        "hmc://chat.harmonyapp.io:2289/fdeded13-844b-42e1-b813-34f74f9afdbc/342";
    const NO_SERVER_HMC: &str = "/fdeded13-844b-42e1-b813-34f74f9afdbc";
    const NO_ID_HMC: &str = "hmc://chat.harmonyapp.io:2289";

    #[test]
    fn parse_valid_hmc() {
        Hmc::try_from(VALID_HMC.parse::<Uri>().unwrap()).unwrap();
    }

    #[test]
    #[should_panic(expected = "InvalidId")]
    fn parse_invalid_id_hmc() {
        if let Err(e) = Hmc::try_from(INVALID_ID_HMC.parse::<Uri>().unwrap()) {
            panic!("{:?}", e)
        }
    }

    #[test]
    #[should_panic(expected = "NoServer")]
    fn parse_no_server_hmc() {
        if let Err(e) = Hmc::try_from(NO_SERVER_HMC.parse::<Uri>().unwrap()) {
            panic!("{:?}", e)
        }
    }

    #[test]
    #[should_panic(expected = "NoId")]
    fn parse_no_id_hmc() {
        if let Err(e) = Hmc::try_from(NO_ID_HMC.parse::<Uri>().unwrap()) {
            panic!("{:?}", e)
        }
    }
}

/// An homeserver identifier containg a domain and a port.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HomeserverIdentifier {
    domain: String,
    port: u16,
}

impl HomeserverIdentifier {
    /// Turn the identifier into the server URL.
    pub fn to_url(&self) -> Uri {
        let mut parts = Parts::default();
        parts.scheme = Some(Scheme::from_str("https").unwrap());
        parts.authority = Some(format!("{}:{}", self.domain, self.port).parse().unwrap());
        Uri::from_parts(parts).unwrap()
    }
}

/// Error thrown when parsing a homeserver ID from a string.
#[derive(Debug, PartialEq, Eq)]
pub enum HomeserverIdParseError {
    /// Port wasn't a `u16`.
    InvalidPort,
    /// Port was missing.
    MissingPort,
    /// Domain was missing.
    MissingDomain,
    /// Homeserver ID was malformed.
    Malformed,
}

impl Display for HomeserverIdParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidPort => "port is invalid (not an u16)",
            Self::MissingPort => "port is missing",
            Self::MissingDomain => "domain is missing",
            Self::Malformed => "id is malformed",
        };
        f.write_str(msg)
    }
}

impl StdError for HomeserverIdParseError {}

impl FromStr for HomeserverIdentifier {
    type Err = HomeserverIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');

        let domain = split.next().ok_or(HomeserverIdParseError::MissingDomain)?;
        if domain.parse::<http::uri::Authority>().is_err() {
            return Err(HomeserverIdParseError::Malformed);
        }
        let port_raw = split.next().ok_or(HomeserverIdParseError::MissingPort)?;

        let port: u16 = port_raw
            .parse()
            .map_err(|_| HomeserverIdParseError::InvalidPort)?;

        Ok(Self {
            domain: domain.to_string(),
            port,
        })
    }
}

/// A trait to facilitate easy request execution.
pub trait Endpoint {
    /// The response type of this endpoint request.
    type Response;

    /// The endpoint path of this request.
    const ENDPOINT_PATH: &'static str;

    #[cfg(feature = "client")]
    /// Execute the request using the provided client.
    fn call_with(
        self,
        client: &crate::client::Client,
    ) -> hrpc::exports::futures_util::future::BoxFuture<
        '_,
        crate::client::error::ClientResult<Self::Response>,
    >;
}
