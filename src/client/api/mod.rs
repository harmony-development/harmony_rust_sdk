/// Auth service client API.
pub mod auth;
/// Chat service client API.
pub mod chat;
/// Emote client API.
pub mod emote;
/// Media proxy service client API.
pub mod mediaproxy;
/// Profile client API.
pub mod profile;
/// REST client API.
pub mod rest;
/// Batch client API.
pub mod batch {}

#[doc(inline)]
pub use crate::api::{harmonytypes, Hmc, HmcFromStrError, HmcParseError};

use crate::{
    client::{Client, ClientResult},
    impl_into_req,
};

use std::fmt::Debug;

use derive_more::{Display, From, Into};
use derive_new::new;
use harmony_derive::{builder, into_request};
use hrpc::{IntoRequest, Request};

#[macro_export]
macro_rules! impl_into_req {
    ($req:ty) => {
        paste::paste! {
            impl IntoRequest<[<$req Request>]> for $req {
                fn into_request(self) -> Request<[<$req Request>]> {
                    [<$req Request>]::from(self).into_request()
                }
            }
        }
    };
}
