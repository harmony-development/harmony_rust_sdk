/// v1 of mediaproxy service.
pub mod v1 {
    #![allow(missing_docs)]
    hrpc::include_proto!("protocol.mediaproxy.v1");
}
pub use v1::*;

impl FetchLinkMetadataRequest {
    /// Create a new [`FetchLinkMetadataRequest`] that fetches one URL.
    #[inline(always)]
    pub fn new_one(url: impl Into<String>) -> Self {
        Self::new(vec![url.into()])
    }
}
