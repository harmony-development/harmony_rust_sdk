/// Auth service client API.
pub mod auth;
/// Chat service client API.
pub mod chat;
/// Media proxy service client API.
pub mod mediaproxy;
/// REST client API.
pub mod rest;

#[doc(inline)]
pub use crate::api::{harmonytypes, Hmc};

use crate::client::{Client, ClientResult};

#[cfg(feature = "request_method")]
use async_trait::async_trait;
use derive_more::{Display, From, Into};
use derive_new::new;
use harmony_derive::{into_request, SelfBuilder};

#[cfg(feature = "request_method")]
#[async_trait]
pub trait ClientRequest<Resp> {
    async fn request(self, client: &Client) -> ClientResult<Resp>;
}

/// This is NOT a part of the public API and should NOT be used.
#[macro_export]
#[doc(hidden)]
macro_rules! client_api {
    {
        $(#[$meta:meta])*
        response: $resp:ty,
        request: $req:ty,
        api_fn: $api_fn:ident,
        service: $service:ident,
    } => {
        $(#[$meta])*
        pub async fn $api_fn<
            Req: ::std::convert::Into<$req> + ::std::fmt::Debug,
        >(client: &$crate::client::Client, request: Req) -> $crate::client::ClientResult<$resp> {
            log::debug!("Sending request: {:?}", request);
            let request = request.into();

            paste::paste! {
                let response = client
                    .[<$service _lock>]()
                    .await
                    .$api_fn (request)
                    .await;
            }
            log::debug!("Received response: {:?}", response);

            response.map_err(::std::convert::Into::into)
        }

        #[cfg(feature = "request_method")]
        #[async_trait]
        impl $crate::client::api::ClientRequest<$resp> for $req {
            async fn request(self, client: &$crate::client::Client) -> $crate::client::ClientResult<$resp> {
                $api_fn(client, self).await
            }
        }
    };
    {
        $(#[$meta:meta])*
        request: $req:ty,
        api_fn: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            response: (),
            request: $req,
            api_fn: $fn_name,
            service: $service,
        }
    };
    {
        $(#[$meta:meta])*
        response: $resp:ty,
        api_fn: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            response: $resp,
            request: (),
            api_fn: $fn_name,
            service: $service,
        }
    };
    {
        $(#[$meta:meta])*
        action: $act:ident,
        api_fn: $fn_name:ident,
        service: $service:ident,
    } => {
        $crate::client_api! {
            $(#[$meta])*
            response: paste::paste! { [<$act Response>] },
            request: paste::paste! { [<$act Request>] },
            api_fn: $fn_name,
            service: $service,
        }
    };
}
