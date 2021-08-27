use super::*;

pub use crate::api::mediaproxy::{
    FetchLinkMetadataRequest, FetchLinkMetadataResponse, InstantViewRequest, SiteMetadata,
};

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

impl IntoRequest<InstantViewRequest> for Url {
    fn into_request(self) -> Request<InstantViewRequest> {
        InstantViewRequest::from(self).into_request()
    }
}

impl IntoRequest<FetchLinkMetadataRequest> for Url {
    fn into_request(self) -> Request<FetchLinkMetadataRequest> {
        FetchLinkMetadataRequest::from(self).into_request()
    }
}
