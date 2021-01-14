use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

use derive_more::{From, Into, IntoIterator};
use http::{uri::Authority, Uri};

/// Chat service API.
#[cfg(feature = "gen_chat")]
pub mod chat;

/// Auth service API.
#[cfg(feature = "gen_auth")]
pub mod auth {
    pub mod v1 {
        tonic::include_proto!("protocol.auth.v1");
    }
    pub use v1::*;
}

/// Common types used in other services.
#[cfg(feature = "gen_harmonytypes")]
pub mod harmonytypes {
    pub mod v1 {
        tonic::include_proto!("protocol.harmonytypes.v1");
    }
    pub use v1::*;
}

/// Media proxy service API.
#[cfg(feature = "gen_mediaproxy")]
pub mod mediaproxy {
    pub mod v1 {
        tonic::include_proto!("protocol.mediaproxy.v1");
    }
    pub use v1::*;
}

/// Voice service API.
#[cfg(feature = "gen_voice")]
pub mod voice {
    pub mod v1 {
        tonic::include_proto!("protocol.voice.v1");
    }
    pub use v1::*;
}

/// Some crates re-exported for user convenience.
pub mod exports {
    pub use http;
    pub use prost;
}

/// Errors that can occur when converting a possibly non-HMC URL to a HMC URL.
#[derive(Debug)]
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

/// A HMC.
///
/// An example HMC looks like `hmc://chat.harmonyapp.io/403cb46c-49cf-4ae1-b876-f38eb26accb0`.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Hmc {
    inner: Uri,
}

impl Hmc {
    /// Creates a new HMC given a homeserver and (attachment) ID.
    ///
    /// Note that this function *does not* check that the `id` arguments is actually an ID,
    /// so it may panic or requests made with this `Hmc` may fail.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::Hmc;
    /// let hmc = Hmc::new("example.org".parse().unwrap(), "403cb46c-49cf-4ae1-b876-f38eb26accb0");
    /// assert_eq!(hmc.to_string(), "hmc://example.org/403cb46c-49cf-4ae1-b876-f38eb26accb0");
    /// ```
    pub fn new(server: Authority, id: impl std::fmt::Display) -> Self {
        let hmc_compliant_uri = Uri::builder()
            .authority(server)
            .path_and_query(format!("/{}", id))
            .scheme("hmc")
            .build()
            .unwrap();

        Self {
            inner: hmc_compliant_uri,
        }
    }

    /// Gets the ID of this HMC.
    ///
    /// # Example
    /// ```
    /// # use harmony_rust_sdk::api::Hmc;
    /// let hmc = Hmc::new("example.org".parse().unwrap(), "403cb46c-49cf-4ae1-b876-f38eb26accb0");
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
    /// let hmc = Hmc::new("example.org".parse().unwrap(), "403cb46c-49cf-4ae1-b876-f38eb26accb0");
    /// assert_eq!(hmc.server(), "example.org");
    /// ```
    pub fn server(&self) -> &str {
        self.inner.authority().unwrap().as_str()
    }
}

impl Display for Hmc {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Into<String> for Hmc {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<Uri> for Hmc {
    type Error = HmcParseError;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        let server = if let Some(authority) = value.authority().cloned() {
            authority
        } else {
            return Err(HmcParseError::NoServer);
        };

        // We trim the first '/' if it exists since it will always be the authority - path seperator
        let path = value.path().trim_start_matches('/');
        let id = if !path.is_empty() {
            if !path.contains('/') {
                path.to_string()
            } else {
                return Err(HmcParseError::InvalidId);
            }
        } else {
            return Err(HmcParseError::NoId);
        };

        Ok(Self::new(server, id))
    }
}

/// Wrapper type for `Vec<Hmc>` so we can implement some traits.
///
/// You don't need to create this manually, since it implements `From<Vec<Hmc>>`.
#[derive(new, Debug, Default, Clone, Into, From, IntoIterator)]
pub struct Hmcs(Vec<Hmc>);

impl Into<Vec<String>> for Hmcs {
    fn into(self) -> Vec<String> {
        self.into_iter().map(|hmc| hmc.to_string()).collect()
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
        Hmc::try_from(Uri::from_static(VALID_HMC)).unwrap();
    }

    #[test]
    #[should_panic(expected = "InvalidId")]
    fn parse_invalid_id_hmc() {
        if let Err(e) = Hmc::try_from(Uri::from_static(INVALID_ID_HMC)) {
            panic!("{:?}", e)
        }
    }

    #[test]
    #[should_panic(expected = "NoServer")]
    fn parse_no_server_hmc() {
        if let Err(e) = Hmc::try_from(Uri::from_static(NO_SERVER_HMC)) {
            panic!("{:?}", e)
        }
    }

    #[test]
    #[should_panic(expected = "NoId")]
    fn parse_no_id_hmc() {
        if let Err(e) = Hmc::try_from(Uri::from_static(NO_ID_HMC)) {
            panic!("{:?}", e)
        }
    }
}
