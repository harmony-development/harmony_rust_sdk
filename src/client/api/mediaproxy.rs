pub use crate::api::mediaproxy::{FetchLinkMetadataRequest, InstantViewRequest, SiteMetadata};

use super::*;
use crate::{api::mediaproxy::*, client_api};

use http::Uri;

impl IntoRequest<FetchLinkMetadataRequest> for Uri {
    fn into_request(self) -> Request<FetchLinkMetadataRequest> {
        FetchLinkMetadataRequest {
            url: self.to_string(),
        }
        .into_request()
    }
}

impl IntoRequest<InstantViewRequest> for Uri {
    fn into_request(self) -> Request<InstantViewRequest> {
        InstantViewRequest {
            url: self.to_string(),
        }
        .into_request()
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
    response: SiteMetadata,
    request: FetchLinkMetadataRequest,
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
