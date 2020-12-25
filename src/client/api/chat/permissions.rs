use super::*;

client_api! {
    /// Get permissions of a role.
    args: {
        guild_id: u64,
        channel_id: u64,
        role_id: u64,
    },
    action: GetPermissions,
    api_func: get_permissions,
    service: chat,
}

client_api! {
    /// Query if a local user (or specified user) has a permission.
    args: {
        guild_id: u64,
        channel_id: u64,
        check_for: String,
        as_user_id: Option<u64>,
    },
    action: QueryPermissions,
    request_fields: {
        r#as: as_user_id.unwrap_or_default(),
        = guild_id, channel_id, check_for,
    },
    api_func: query_has_permission,
    service: chat,
}

client_api! {
    /// Set permissions of a role.
    args: {
        guild_id: u64,
        channel_id: u64,
        role_id: u64,
        permissions: PermissionList,
    },
    request: SetPermissionsRequest {
        guild_id, channel_id, role_id,
        perms: Some(permissions),
    },
    api_func: set_permissions,
    service: chat,
}

client_api! {
    /// Get a list of all roles in a guild.
    args: { guild_id: u64, },
    action: GetGuildRoles,
    api_func: get_guild_roles,
    service: chat,
}

client_api! {
    /// Add a role to a guild.
    args: { guild_id: u64, role: Role, },
    action: AddGuildRole,
    request_fields: {
        role: Some(role),
        = guild_id,
    },
    api_func: add_guild_role,
    service: chat,
}

client_api! {
    /// Delete a role in a guild.
    args: { guild_id: u64, role_id: u64, },
    request_type: DeleteGuildRoleRequest,
    api_func: delete_guild_role,
    service: chat,
}

client_api! {
    /// Modify a role in a guild.
    args: {
        guild_id: u64,
        role: Role,
        modify_name: bool,
        modify_color: bool,
        modify_hoist: bool,
        modify_pingable: bool,
    },
    request: ModifyGuildRoleRequest {
        role: Some(role),
        guild_id,
        modify_name,
        modify_color,
        modify_hoist,
        modify_pingable,
    },
    api_func: modify_guild_role,
    service: chat,
}

client_api! {
    /// Move a role to somewhere else on the role list.
    args: {
        guild_id: u64,
        role_id: u64,
        new_role_place: Place,
    },
    action: MoveRole,
    request_fields: {
        before_id: new_role_place.next(),
        after_id: new_role_place.previous(),
        = guild_id, role_id,
    },
    api_func: move_role,
    service: chat,
}

client_api! {
    /// Manage a user's roles.
    args: {
        guild_id: u64,
        user_id: u64,
        give_role_ids: Vec<u64>,
        take_role_ids: Vec<u64>,
    },
    request_type: ManageUserRolesRequest,
    api_func: manage_user_roles,
    service: chat,
}

client_api! {
    /// Get a list of all roles a user has.
    args: {
        guild_id: u64,
        user_id: u64,
    },
    action: GetUserRoles,
    api_func: get_user_roles,
    service: chat,
}
