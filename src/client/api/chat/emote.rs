pub use crate::api::chat::{
    AddEmoteToPackRequest, CreateEmotePackRequest, DeleteEmoteFromPackRequest,
    DeleteEmotePackRequest, DequipEmotePackRequest, GetEmotePackEmotesRequest,
    GetEmotePacksRequest,
};
use crate::client::api::rest::FileId;

use super::*;

/// Wrapper around an emote pack ID which can be used as multiple requests.
#[into_request(
    "GetEmotePackEmotesRequest",
    "DeleteEmotePackRequest",
    "DequipEmotePackRequest"
)]
#[derive(Debug, Clone, new)]
pub struct PackId {
    pack_id: u64,
}

/// Convenience type to create a valid [`CreateEmotePackRequest`].
#[into_request("CreateEmotePackRequest")]
#[derive(Debug, Clone, new)]
pub struct CreateEmotePack {
    pack_name: String,
}

/// Convenience type to create a valid [`AddEmoteToPackRequest`].
#[into_request("AddEmoteToPackRequest")]
#[derive(Debug, Clone, new)]
pub struct AddEmoteToPack {
    pack_id: u64,
    image_id: FileId,
    name: String,
}

/// Convenience type to create a valid [`DeleteEmoteFromPackRequest`].
#[into_request("DeleteEmoteFromPackRequest")]
#[derive(Debug, Clone, new)]
pub struct DeleteEmoteFromPack {
    pack_id: u64,
    image_id: FileId,
}
