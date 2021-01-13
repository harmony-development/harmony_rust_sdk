use super::*;

/// Convenience type to create a valid [`GetPermissionsRequest`].
#[into_request("GetPermissionsRequest")]
#[derive(Debug, new)]
pub struct GetPermissions {
    guild_id: u64,
    channel_id: u64,
    role_id: u64,
}

client_api! {
    /// Get permissions of a role.
    action: GetPermissions,
    api_fn: get_permissions,
    service: chat,
}

/// Convenience type to create a valid [`QueryPermissionsRequest`].
#[into_request("QueryPermissionsRequest")]
#[derive(Debug, new, SelfBuilder)]
pub struct QueryPermissions {
    guild_id: u64,
    channel_id: u64,
    check_for: String,
    #[new(default)]
    r#as: u64,
}

client_api! {
    /// Query if a local user (or specified user) has a permission.
    action: QueryPermissions,
    api_fn: query_has_permission,
    service: chat,
}

/// Convenience type to create a valid [`SetPermissionsRequest`].
#[into_request("SetPermissionsRequest")]
#[derive(Debug, SelfBuilder, new)]
pub struct SetPermissions {
    guild_id: u64,
    channel_id: u64,
    role_id: u64,
    #[new(default)]
    #[builder(setter(strip_option))]
    perms: Option<PermissionList>,
}

client_api! {
    /// Set permissions of a role.
    request: SetPermissionsRequest,
    api_fn: set_permissions,
    service: chat,
}

client_api! {
    /// Get a list of all roles in a guild.
    action: GetGuildRoles,
    api_fn: get_guild_roles,
    service: chat,
}

/// Convenience type to create a valid [`AddGuildRoleRequest`].
#[derive(Debug, new)]
pub struct AddGuildRole {
    guild_id: u64,
    role: Role,
}

impl IntoRequest<AddGuildRoleRequest> for AddGuildRole {
    fn into_request(self) -> Request<AddGuildRoleRequest> {
        AddGuildRoleRequest {
            guild_id: self.guild_id,
            role: Some(self.role),
        }
        .into_request()
    }
}

client_api! {
    /// Add a role to a guild.
    action: AddGuildRole,
    api_fn: add_guild_role,
    service: chat,
}

/// Convenience type to create a valid [`DeleteGuildRoleRequest`].
#[into_request("DeleteGuildRoleRequest")]
#[derive(Debug, new)]
pub struct DeleteGuildRole {
    guild_id: u64,
    role_id: u64,
}

client_api! {
    /// Delete a role in a guild.
    request: DeleteGuildRoleRequest,
    api_fn: delete_guild_role,
    service: chat,
}

// TODO: Make a `RoleUpdate` struct for this
/// Convenience type to create a valid [`ModifyGuildRoleRequest`].
#[derive(Debug, new, SelfBuilder)]
pub struct ModifyGuildRole {
    guild_id: u64,
    role: Role,
    #[new(default)]
    modify_name: bool,
    #[new(default)]
    modify_color: bool,
    #[new(default)]
    modify_hoist: bool,
    #[new(default)]
    modify_pingable: bool,
}

impl IntoRequest<ModifyGuildRoleRequest> for ModifyGuildRole {
    fn into_request(self) -> Request<ModifyGuildRoleRequest> {
        ModifyGuildRoleRequest {
            guild_id: self.guild_id,
            modify_name: self.modify_name,
            modify_color: self.modify_color,
            modify_hoist: self.modify_hoist,
            modify_pingable: self.modify_pingable,
            role: Some(self.role),
        }
        .into_request()
    }
}

client_api! {
    /// Modify a role in a guild.
    request: ModifyGuildRoleRequest,
    api_fn: modify_guild_role,
    service: chat,
}

/// Convenience type to create a valid [`MoveRoleRequest`].
#[derive(Debug, new)]
pub struct MoveRole {
    guild_id: u64,
    role_id: u64,
    new_role_place: Place,
}

impl IntoRequest<MoveRoleRequest> for MoveRole {
    fn into_request(self) -> Request<MoveRoleRequest> {
        MoveRoleRequest {
            guild_id: self.guild_id,
            role_id: self.role_id,
            before_id: self.new_role_place.next(),
            after_id: self.new_role_place.previous(),
        }
        .into_request()
    }
}

client_api! {
    /// Move a role to somewhere else on the role list.
    action: MoveRole,
    api_fn: move_role,
    service: chat,
}

/// Convenience type to create a valid [`ManageUserRolesRequest`].
#[into_request("ManageUserRolesRequest")]
#[derive(Debug, new, SelfBuilder)]
pub struct ManageUserRoles {
    guild_id: u64,
    user_id: u64,
    #[new(default)]
    give_role_ids: Vec<u64>,
    #[new(default)]
    take_role_ids: Vec<u64>,
}

client_api! {
    /// Manage a user's roles.
    request: ManageUserRolesRequest,
    api_fn: manage_user_roles,
    service: chat,
}

/// Convenience type to create a valid [`GetUserRolesRequest`].
#[into_request("GetUserRolesRequest")]
#[derive(Debug, new)]
pub struct GetUserRoles {
    guild_id: u64,
    user_id: u64,
}

client_api! {
    /// Get a list of all roles a user has.
    action: GetUserRoles,
    api_fn: get_user_roles,
    service: chat,
}
