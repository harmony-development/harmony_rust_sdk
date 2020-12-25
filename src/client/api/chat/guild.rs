use super::*;

client_api! {
    /// Get guild list for local user.
    action: GetGuildList,
    api_func: get_guild_list,
    service: chat,
}

client_api! {
    /// Get guild data of a guild.
    args: { guild_id: u64, },
    action: GetGuild,
    api_func: get_guild,
    service: chat,
}

client_api! {
    /// Create a new guild.
    args: { name: String, picture_url: Option<Uri>, },
    action: CreateGuild,
    request_fields: {
        guild_name: name,
        picture_url: picture_url.map_or_else(String::default, |u| u.to_string()),
    },
    api_func: create_guild,
    service: chat,
}

client_api! {
    /// Add a guild to the guild list.
    args: { guild_id: u64, homeserver: Uri, },
    action: AddGuildToGuildList,
    request_fields: {
        homeserver: homeserver.to_string(),
        = guild_id,
    },
    api_func: add_guild_to_guild_list,
    service: chat,
}

client_api! {
    /// Remove a guild from the guild list.
    args: { guild_id: u64, homeserver: Uri, },
    action: RemoveGuildFromGuildList,
    request_fields: {
        homeserver: homeserver.to_string(),
        = guild_id,
    },
    api_func: remove_guild_from_guild_list,
    service: chat,
}

client_api! {
    /// Update a guild's information.
    args: {
        guild_id: u64,
        new_guild_name: Option<String>,
        new_guild_picture: Option<Uri>,
    },
    request: UpdateGuildInformationRequest {
        guild_id,
        update_guild_name: new_guild_name.is_some(),
        update_guild_picture: new_guild_picture.is_some(),
        new_guild_name: new_guild_name.unwrap_or_default(),
        new_guild_picture: new_guild_picture.map(|u| u.to_string()).unwrap_or_default(),
    },
    api_func: update_guild_information,
    service: chat,
}

client_api! {
    /// Delete a guild.
    args: { guild_id: u64, },
    request_type: DeleteGuildRequest,
    api_func: delete_guild,
    service: chat,
}

client_api! {
    /// Join a guild, using the specified invite id.
    args: { invite_id: InviteId, },
    action: JoinGuild,
    request_fields: {
        invite_id: invite_id.into_name(),
    },
    api_func: join_guild,
    service: chat,
}

client_api! {
    /// Leave a guild.
    args: { guild_id: u64, },
    request_type: LeaveGuildRequest,
    api_func: leave_guild,
    service: chat,
}
