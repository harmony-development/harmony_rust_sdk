pub mod core;
pub mod foundation;
pub mod profile;

type Unit = ();

#[macro_export]
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
                    let request = $req {
                        $(
                            $req_arg: $convert($arg_name),
                        )*
                    };

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

#[macro_export]
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
