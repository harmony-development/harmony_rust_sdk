/// Auth service client API.
pub mod auth;
/// Chat service client API.
pub mod chat;
/// Media proxy service client API.
pub mod mediaproxy;

use crate::client::{Client, ClientResult};
use tonic::{Request, Response};

// Re export common types
pub use crate::api::harmonytypes::{r#override::Reason, *};

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

                    if let $crate::client::AuthStatus::Complete(session) = &*client.auth_status_lock() {
                        // Session session_token should be ASCII, so this unwrap won't panic
                        request.metadata_mut().insert("auth", session.session_token.parse().unwrap());
                    }

                    log::debug!("Sending request: {:?}", request);
                    let response = client
                        .[<$service _lock>]()
                        .$fn_name (request)
                        .await;
                    log::debug!("Got response: {:?}", response);

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
