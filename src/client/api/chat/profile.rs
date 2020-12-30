use super::*;

client_api! {
    /// Get a list of all users in a guild.
    args: { guild_id: u64, },
    action: GetGuildMembers,
    api_func: get_guild_members,
    service: chat,
}

client_api! {
    /// Get a user's profile.
    args: { user_id: u64, },
    action: GetUser,
    api_func: get_user,
    service: chat,
}

client_api! {
    /// Get a user's metadata.
    args: { app_id: String, },
    action: GetUserMetadata,
    api_func: get_user_metadata,
    service: chat,
}

client_api! {
    /// Update local user's profile.
    args: {
        new_username: Option<String>,
        new_status: Option<UserStatus>,
        new_avatar: Option<Uri>,
        new_is_bot: Option<bool>,
    },
    request: ProfileUpdateRequest {
        update_username: new_username.is_some(),
        update_status: new_status.is_some(),
        update_avatar: new_avatar.is_some(),
        update_is_bot: new_is_bot.is_some(),
        new_username: new_username.unwrap_or_default(),
        new_status: new_status.unwrap_or_default().into(),
        new_avatar: new_avatar.map(|u| u.to_string()).unwrap_or_default(),
        is_bot: new_is_bot.unwrap_or_default(),
    },
    api_func: profile_update,
    service: chat,
}
