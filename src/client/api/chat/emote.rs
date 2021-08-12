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

client_api! {
    /// Create a new emote pack.
    action: CreateEmotePack,
    api_fn: create_emote_pack,
    service: chat,
}

client_api! {
    /// Get a list of all emote packs.
    action: GetEmotePacks,
    api_fn: get_emote_packs,
    service: chat,
}

client_api! {
    /// Get a list of all emotes in an emote pack.
    action: GetEmotePackEmotes,
    api_fn: get_emote_pack_emotes,
    service: chat,
}

/// Convenience type to create a valid [`AddEmoteToPackRequest`].
#[into_request("AddEmoteToPackRequest")]
#[derive(Debug, Clone, new)]
pub struct AddEmoteToPack {
    pack_id: u64,
    image_id: FileId,
    name: String,
}

client_api! {
    /// Add an emote to an emote pack.
    request: AddEmoteToPackRequest,
    api_fn: add_emote_to_pack,
    service: chat,
}

/// Convenience type to create a valid [`DeleteEmoteFromPackRequest`].
#[into_request("DeleteEmoteFromPackRequest")]
#[derive(Debug, Clone, new)]
pub struct DeleteEmoteFromPack {
    pack_id: u64,
    image_id: FileId,
}

client_api! {
    /// Delete an emote from an emote pack.
    request: DeleteEmoteFromPackRequest,
    api_fn: delete_emote_from_pack,
    service: chat,
}

client_api! {
    /// Delete an emote pack.
    request: DeleteEmotePackRequest,
    api_fn: delete_emote_pack,
    service: chat,
}

client_api! {
    /// Dequip an emote pack.
    request: DequipEmotePackRequest,
    api_fn: dequip_emote_pack,
    service: chat,
}

client_api! {
    /// Equip an emote pack.
    request: EquipEmotePackRequest,
    api_fn: equip_emote_pack,
    service: chat,
}
