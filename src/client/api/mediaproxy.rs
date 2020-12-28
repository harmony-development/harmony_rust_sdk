use super::*;

use crate::{api::mediaproxy::*, client_api};
use http::Uri;

pub use crate::api::mediaproxy::SiteMetadata;

client_api! {
    /// Request an Instant View from the server.
    args: { url: Uri, },
    action: InstantView,
    request_fields: {
        url: url.to_string(),
    },
    api_func: instant_view,
    service: mediaproxy,
}

client_api! {
    /// Request a link's (site) metadata from the server.
    args: { url: Uri, },
    response: SiteMetadata,
    request: FetchLinkMetadataRequest {
        url: url.to_string(),
    },
    api_func: fetch_link_metadata,
    service: mediaproxy,
}

client_api! {
    /// Ask the server if it can provide an Instant View for a specified URL.
    args: { url: Uri, },
    response: CanInstantViewResponse,
    request: InstantViewRequest {
        url: url.to_string(),
    },
    api_func: can_instant_view,
    service: mediaproxy,
}
