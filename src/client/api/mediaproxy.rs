pub use crate::api::mediaproxy::{
    FetchLinkMetadataRequest, FetchLinkMetadataResponse, InstantViewRequest, SiteMetadata,
};

use super::*;
use crate::{api::mediaproxy::*, client_api};

use hrpc::url::Url;

impl From<Url> for FetchLinkMetadataRequest {
    fn from(o: Url) -> FetchLinkMetadataRequest {
        FetchLinkMetadataRequest { url: o.to_string() }
    }
}

impl From<Url> for InstantViewRequest {
    fn from(o: Url) -> InstantViewRequest {
        InstantViewRequest { url: o.to_string() }
    }
}

client_api! {
    /// Request an Instant View from the server.
    action: InstantView,
    api_fn: instant_view,
    service: mediaproxy,
}

client_api! {
    /// Request a link's (site) metadata from the server.
    action: FetchLinkMetadata,
    api_fn: fetch_link_metadata,
    service: mediaproxy,
}

client_api! {
    /// Ask the server if it can provide an Instant View for a specified URL.
    response: CanInstantViewResponse,
    request: InstantViewRequest,
    api_fn: can_instant_view,
    service: mediaproxy,
}
