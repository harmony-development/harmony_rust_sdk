use super::*;

pub use crate::api::mediaproxy::{
    FetchLinkMetadataRequest, FetchLinkMetadataResponse, InstantViewRequest, SiteMetadata,
};

use http::Uri;

impl From<Uri> for FetchLinkMetadataRequest {
    fn from(o: Uri) -> FetchLinkMetadataRequest {
        FetchLinkMetadataRequest { url: o.to_string() }
    }
}

impl From<Uri> for InstantViewRequest {
    fn from(o: Uri) -> InstantViewRequest {
        InstantViewRequest { url: o.to_string() }
    }
}

impl IntoRequest<InstantViewRequest> for Uri {
    fn into_request(self) -> Request<InstantViewRequest> {
        InstantViewRequest::from(self).into_request()
    }
}

impl IntoRequest<FetchLinkMetadataRequest> for Uri {
    fn into_request(self) -> Request<FetchLinkMetadataRequest> {
        FetchLinkMetadataRequest::from(self).into_request()
    }
}
