pub use crate::api::chat::{
    AddGuildRoleRequest, DeleteGuildRoleRequest, GetGuildRolesRequest, GetPermissionsRequest,
    GetUserRolesRequest, ManageUserRolesRequest, ModifyGuildRoleRequest, MoveRoleRequest,
    Permission, QueryHasPermissionRequest, SetPermissionsRequest,
};

use super::*;

/// Convenience type to create a valid [`GetPermissionsRequest`].
#[into_request("GetPermissionsRequest")]
#[derive(Debug, Clone, new, builder)]
pub struct GetPermissions {
    guild_id: u64,
    #[new(default)]
    channel_id: u64,
    role_id: u64,
}

/// Convenience type to create a valid [`QueryHasPermissionRequest`].
#[into_request("QueryHasPermissionRequest")]
#[derive(Debug, Clone, new, builder)]
pub struct QueryPermissions {
    guild_id: u64,
    #[new(default)]
    channel_id: u64,
    check_for: String,
    #[new(default)]
    r#as: u64,
}

/// Convenience type to create a valid [`SetPermissionsRequest`].
#[into_request("SetPermissionsRequest")]
#[derive(Debug, Clone, builder, new)]
pub struct SetPermissions {
    guild_id: u64,
    #[new(default)]
    channel_id: u64,
    role_id: u64,
    #[new(default)]
    perms_to_give: Vec<Permission>,
}

/// Convenience type to create a valid [`AddGuildRoleRequest`].
#[into_request("AddGuildRoleRequest")]
#[derive(Debug, Clone, builder, new)]
pub struct AddGuildRole {
    guild_id: u64,
    name: String,
    #[new(default)]
    color: i32,
    #[new(default)]
    hoist: bool,
    #[new(default)]
    pingable: bool,
}

/// Convenience type to create a valid [`DeleteGuildRoleRequest`].
#[into_request("DeleteGuildRoleRequest")]
#[derive(Debug, Clone, new)]
pub struct DeleteGuildRole {
    guild_id: u64,
    role_id: u64,
}

// TODO: Make a `RoleUpdate` struct for this
/// Convenience type to create a valid [`ModifyGuildRoleRequest`].
#[into_request("ModifyGuildRoleRequest")]
#[derive(Debug, Clone, new, builder)]
pub struct ModifyGuildRole {
    guild_id: u64,
    role_id: u64,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_name: Option<String>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_color: Option<i32>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_hoist: Option<bool>,
    #[builder(setter(strip_option))]
    #[new(default)]
    new_pingable: Option<bool>,
}

/// Convenience type to create a valid [`MoveRoleRequest`].
#[into_request("MoveRoleRequest")]
#[derive(Debug, Clone, new)]
pub struct MoveRole {
    guild_id: u64,
    role_id: u64,
    new_position: Place,
}

/// Convenience type to create a valid [`ManageUserRolesRequest`].
#[into_request("ManageUserRolesRequest")]
#[derive(Debug, Clone, new, builder)]
pub struct ManageUserRoles {
    guild_id: u64,
    user_id: u64,
    #[new(default)]
    give_role_ids: Vec<u64>,
    #[new(default)]
    take_role_ids: Vec<u64>,
}

/// Convenience type to create a valid [`GetUserRolesRequest`].
#[into_request("GetUserRolesRequest")]
#[derive(Debug, Clone, new)]
pub struct GetUserRoles {
    guild_id: u64,
    user_id: u64,
}
