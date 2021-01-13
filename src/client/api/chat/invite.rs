use super::*;

client_api! {
    /// Get invites for the specified guild.
    action: GetGuildInvites,
    api_fn: get_guild_invites,
    service: chat,
}

/// Convenience type to create a valid [`CreateInviteRequest`].
#[into_request("CreateInviteRequest")]
#[derive(Debug, new)]
pub struct CreateInvite {
    name: InviteId,
    possible_uses: i32,
    guild_id: u64,
}

client_api! {
    /// Create an invite with a name and a number of uses for the specified guild.
    ///
    /// If the number of possible uses are `0`, invite usage won't be limited.
    action: CreateInvite,
    api_fn: create_invite,
    service: chat,
}

/// Convenience type to create a valid [`DeleteInviteRequest`].
#[into_request("DeleteInviteRequest")]
#[derive(Debug, new)]
pub struct DeleteInvite {
    invite_id: InviteId,
    guild_id: u64,
}

client_api! {
    /// Delete an invite with the specified name in the specified guild.
    request: DeleteInviteRequest,
    api_fn: delete_invite,
    service: chat,
}
