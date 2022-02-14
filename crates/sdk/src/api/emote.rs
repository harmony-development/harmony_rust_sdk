/// v1 of emote service.
pub mod v1 {
    #![allow(missing_docs)]
    hrpc::include_proto!("protocol.emote.v1");
}
pub use v1::*;

impl GetEmotePackEmotesRequest {
    /// Create a new [`GetEmotePackEmotesRequest`] for fetching one emote
    /// pack's emotes.
    #[inline(always)]
    pub fn new_one(pack_id: u64) -> Self {
        Self::new(vec![pack_id])
    }
}
