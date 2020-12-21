pub mod core;
pub mod foundation;
pub mod profile;

type Unit = ();

/// This is NOT a part of the public API and should NOT be used.
#[macro_export]
#[doc(hidden)]
macro_rules! client_api {
    {
        api_func: $fn_name:ident,
        service: $service:ident,
        resp: $resp:ident,
        req: $req:ident,

        $(#[$meta:meta])*
        args {
            $( $arg_name:ident: $arg_type:ty => $req_arg:ident: $convert:expr; )*
        }
    } => {
        paste::paste! {
            $(#[$meta])*
            pub async fn $fn_name (client: &Client, $( $arg_name: $arg_type, )*) -> ClientResult<$resp> {
                    let mut request = Request::new($req {
                        $(
                            $req_arg: $convert($arg_name),
                        )*
                    });

                    if let Some(session) = &*client.session_lock() {
                        // Session access_token should be ASCII, so this unwrap won't panic
                        request.metadata_mut().insert("auth", session.session_token.parse().unwrap());
                    }

                    client
                        .[<$service _lock>]()
                        .$fn_name (request)
                        .await
                        .map(Response::into_inner)
                        .map_err(Into::into)
            }
        }
    };
}

/// This is NOT a part of the public API and should NOT be used.
#[macro_export]
#[doc(hidden)]
macro_rules! client_api_action {
    {
        api_func: $fn_name:ident,
        service: $service:ident,
        action: $action:ident,

        $(#[$meta:meta])*
        args {
            $( $arg_name:ident: $arg_type:ty => $req_arg:ident: $convert:expr; )*
        }
    } => {
        paste::paste! {
            $crate::client_api! {
                api_func: $fn_name,
                service: $service,
                resp: [<$action Response>],
                req: [<$action Request>],

                $(#[$meta])*
                args {
                    $( $arg_name: $arg_type => $req_arg: $convert; )*
                }
            }
        }
    }
}
