pub use crate::api::chat::{
    CreateInviteRequest, DeleteInviteRequest, GetGuildInvitesRequest, InviteId,
};

use super::*;

/// Convenience type to create a valid [`CreateInviteRequest`].
#[into_request("CreateInviteRequest")]
#[derive(Debug, Clone, new)]
pub struct CreateInvite {
    name: InviteId,
    possible_uses: u32,
    guild_id: u64,
}

/// Convenience type to create a valid [`DeleteInviteRequest`].
#[into_request("DeleteInviteRequest")]
#[derive(Debug, Clone, new)]
pub struct DeleteInvite {
    invite_id: InviteId,
    guild_id: u64,
}
