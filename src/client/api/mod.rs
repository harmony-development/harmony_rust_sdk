/// Auth service client API.
pub mod auth;
/// Chat service client API.
pub mod chat;
/// Media proxy service client API.
pub mod mediaproxy;
/// REST client API.
pub mod rest;

#[doc(inline)]
pub use crate::api::{harmonytypes, Hmc, HmcFromStrError, HmcParseError};

use crate::client::{Client, ClientResult};

use std::fmt::Debug;

use derive_more::{Display, From, Into};
use derive_new::new;
use harmony_derive::{into_request, SelfBuilder};

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
        pub async fn $api_fn<Req: Into<$req> + Debug>(client: &Client, request: Req) -> ClientResult<$resp> {
            use tracing::Instrument;

            client
                .generic_api_fn(|c, r| async move { c.$service().await.$api_fn(r).await }, request)
                .instrument(tracing::debug_span!(stringify!($req)))
                .await
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
