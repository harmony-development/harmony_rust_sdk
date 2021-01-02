/// Auth service client API.
pub mod auth;
/// Chat service client API.
pub mod chat;
/// Media proxy service client API.
pub mod mediaproxy;
/// REST client API.
pub mod rest;

use crate::client::{Client, ClientResult};

use http::{uri::Authority, Uri};
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use tonic::{Request, Response};

// Re export common types
pub use crate::api::harmonytypes::{r#override::Reason, *};

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
#[derive(Debug, Clone)]
pub struct Hmc {
    inner: Uri,
}

impl Hmc {
    /// Creates a new HMC given a homeserver and (attachment) ID.
    ///
    /// Note that this function *does not* check that the `id` arguments is actually an ID,
    /// so it may panic or requests made with this `Hmc` may fail.
    pub fn new(server: Authority, id: String) -> Self {
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
    pub fn id(&self) -> &str {
        self.inner.path().trim_start_matches('/')
    }

    /// Gets the server of this HMC.
    pub fn server(&self) -> &str {
        self.inner.authority().unwrap().as_str()
    }
}

impl Display for Hmc {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
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

/// This is NOT a part of the public API and should NOT be used.
#[macro_export]
#[doc(hidden)]
macro_rules! client_api {
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        response: $resp:ty,
        request: $req:expr,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        paste::paste! {
            $(#[$meta])*
            pub async fn $fn_name (client: &Client, $( $arg_name: $arg_type, )*) -> ClientResult<$resp> {
                    let mut request = Request::new($req);

                    if let $crate::client::AuthStatus::Complete(session) = client.auth_status() {
                        // Session session_token should be ASCII, so this unwrap won't panic
                        request.metadata_mut().insert("auth", session.session_token.parse().unwrap());
                    }

                    log::debug!("Sending request");
                    let response = client
                        .[<$service _lock>]()
                        .$fn_name (request)
                        .await;
                    log::debug!("Received response: {:?}", response);

                    response
                        .map(Response::into_inner)
                        .map_err(Into::into)
            }
        }
    };
    {
        $(#[$meta:meta])*
        response: $resp:ty,
        request: $req:expr,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            args: { },
            response: $resp,
            request: $req,
            api_func: $fn_name,
            service: $service,
        }
    };
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        request: $req:expr,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            args: {
                $( $arg_name: $arg_type, )*
            },
            response: (),
            request: $req,
            api_func: $fn_name,
            service: $service,
        }
    };
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        request_type: $req:ident,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            args: {
                $( $arg_name: $arg_type, )*
            },
            response: (),
            request: ($req {
                $( $arg_name, )*
            }),
            api_func: $fn_name,
            service: $service,
        }
    };
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        response: $resp:ty,
        request_type: $req:ident,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            args: {
                $( $arg_name: $arg_type, )*
            },
            response: $resp,
            request: ($req {
                $( $arg_name, )*
            }),
            api_func: $fn_name,
            service: $service,
        }
    };
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        action: $act:ident,
        request_fields: {
            $( $field_name:ident: $field_value:expr, )*
            = $( $field:ident, )*
        },
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        paste::paste! {
            $crate::client_api! {
                $(#[$meta])*
                args: {
                    $( $arg_name: $arg_type, )*
                },
                response: [<$act Response>],
                request: ([<$act Request>] {
                    $( $field_name: $field_value, )*
                    $( $field, )*
                }),
                api_func: $fn_name,
                service: $service,
            }
        }
    };
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        action: $act:ident,
        request_fields: {
            $( $field_name:ident: $field_value:expr, )*
        },
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        paste::paste! {
            $crate::client_api! {
                $(#[$meta])*
                args: {
                    $( $arg_name: $arg_type, )*
                },
                response: [<$act Response>],
                request: ([<$act Request>] {
                    $( $field_name: $field_value, )*
                }),
                api_func: $fn_name,
                service: $service,
            }
        }
    };
    {
        $(#[$meta:meta])*
        args: {
            $( $arg_name:ident: $arg_type:ty, )*
        },
        action: $act:ident,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        paste::paste! {
            $crate::client_api! {
                $(#[$meta])*
                args: {
                    $( $arg_name: $arg_type, )*
                },
                response: [<$act Response>],
                request: ([<$act Request>] {
                    $( $arg_name, )*
                }),
                api_func: $fn_name,
                service: $service,
            }
        }
    };
    {
        $(#[$meta:meta])*
        action: $act:ident,
        api_func: $fn_name:ident,
        service: $service:ident,
    } => {
        paste::paste! {
            $crate::client_api! {
                $(#[$meta])*
                args: { },
                action: $act,
                api_func: $fn_name,
                service: $service,
            }
        }
    };
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
        match Hmc::try_from(Uri::from_static(INVALID_ID_HMC)) {
            Err(e) => panic!("{:?}", e),
            _ => {}
        }
    }

    #[test]
    #[should_panic(expected = "NoServer")]
    fn parse_no_server_hmc() {
        match Hmc::try_from(Uri::from_static(NO_SERVER_HMC)) {
            Err(e) => panic!("{:?}", e),
            _ => {}
        }
    }

    #[test]
    #[should_panic(expected = "NoId")]
    fn parse_no_id_hmc() {
        match Hmc::try_from(Uri::from_static(NO_ID_HMC)) {
            Err(e) => panic!("{:?}", e),
            _ => {}
        }
    }
}
