use super::*;

client_api! {
    /// Get invites for the specified guild.
    args: { guild_id: u64, },
    action: GetGuildInvites,
    api_func: get_guild_invites,
    service: chat,
}

client_api! {
    /// Create an invite with a name and a number of uses for the specified guild.
    ///
    /// If the number of possible uses are `0`, invite usage won't be limited.
    args: {
        name: InviteId,
        possible_uses: u32,
        guild_id: u64,
    },
    action: CreateInvite,
    request_fields: {
        possible_uses: if possible_uses == 0 { -1 } else { possible_uses as i32 },
        name: name.into_name(),
        = guild_id,
    },
    api_func: create_invite,
    service: chat,
}

client_api! {
    /// Delete an invite with the specified name in the specified guild.
    args: { guild_id: u64, invite_id: InviteId, },
    request: DeleteInviteRequest {
        invite_id: invite_id.into_name(),
        guild_id,
    },
    api_func: delete_invite,
    service: chat,
}
