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

use crate::client::{Client, ClientResult};

use std::fmt::Debug;

use derive_more::{Display, From, Into};
use derive_new::new;
use harmony_derive::{impl_call_action, impl_into_req_from, into_request, self_builder};
use hrpc::request::{IntoRequest, Request};
