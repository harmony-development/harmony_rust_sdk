use super::*;

client_api! {
    /// Create a new emote pack.
    args: { pack_name: String, },
    action: CreateEmotePack,
    api_func: create_emote_pack,
    service: chat,
}

client_api! {
    /// Get a list of all emote packs.
    action: GetEmotePacks,
    api_func: get_emote_packs,
    service: chat,
}

client_api! {
    /// Get a list of all emotes in an emote pack.
    args: { pack_id: u64, },
    action: GetEmotePackEmotes,
    api_func: get_emote_pack_emotes,
    service: chat,
}

client_api! {
    /// Add an emote to an emote pack.
    args: {
        pack_id: u64,
        image_id: Uri,
        name: String,
    },
    request: AddEmoteToPackRequest {
        image_id: image_id.to_string(),
        pack_id, name,
    },
    api_func: add_emote_to_pack,
    service: chat,
}

client_api! {
    /// Delete an emote from an emote pack.
    args: {
        pack_id: u64,
        image_id: Uri,
    },
    request: DeleteEmoteFromPackRequest {
        image_id: image_id.to_string(),
        pack_id,
    },
    api_func: delete_emote_from_pack,
    service: chat,
}

client_api! {
    /// Delete an emote pack.
    args: { pack_id: u64, },
    request_type: DeleteEmotePackRequest,
    api_func: delete_emote_pack,
    service: chat,
}

client_api! {
    /// Dequip an emote pack.
    args: { pack_id: u64, },
    request_type: DequipEmotePackRequest,
    api_func: dequip_emote_pack,
    service: chat,
}
