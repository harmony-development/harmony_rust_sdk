use super::Unit;
use crate::{
    api::core::*,
    client::{Client, ClientResult},
    client_api, client_api_action,
};
use http::Uri;
use tonic::{Request, Response};

// Export everything a client may need for this service
pub use crate::api::core::{
    event, get_emote_pack_emotes_response::Emote, get_emote_packs_response::EmotePack,
    get_guild_channels_response::Channel, get_guild_invites_response::Invite,
    get_guild_list_response::GuildListEntry, permission::Mode, r#override::Reason,
    stream_events_request, Action, ActionPresentation, ActionType, Embed, Event, FieldPresentation,
    Override, PermissionList, Role,
};

// GUILD

client_api_action! {
    api_func: create_guild,
    service: core,
    action: CreateGuild,

    args {
        name: String => guild_name: (|n| n);
        picture_url: Option<Uri> => picture_url: (|u: Option<Uri>| u.map_or_else(String::default, |u| u.to_string()));
    }
}

client_api_action! {
    api_func: get_guild,
    service: core,
    action: GetGuild,

    args {
        guild_id: u64 => guild_id: (|g| g);
    }
}

client_api_action! {
    api_func: get_guild_invites,
    service: core,
    action: GetGuildInvites,

    args {
        guild_id: u64 => guild_id: (|g| g);
    }
}

client_api_action! {
    api_func: get_guild_members,
    service: core,
    action: GetGuildMembers,

    args {
        guild_id: u64 => guild_id: (|g| g);
    }
}

client_api_action! {
    api_func: get_guild_channels,
    service: core,
    action: GetGuildChannels,

    args {
        guild_id: u64 => guild_id: (|g| g);
    }
}

client_api_action! {
    api_func: get_guild_list,
    service: core,
    action: GetGuildList,

    args { }
}

client_api_action! {
    api_func: add_guild_to_guild_list,
    service: core,
    action: AddGuildToGuildList,

    args {
        guild_id: u64 => guild_id: (|i| i);
        homeserver: Uri => homeserver: (|h: Uri| h.to_string());
    }
}

client_api_action! {
    api_func: remove_guild_from_guild_list,
    service: core,
    action: RemoveGuildFromGuildList,

    args {
        guild_id: u64 => guild_id: (|u| u);
        homeserver: Uri => homeserver: (|h: Uri| h.to_string());
    }
}

client_api! {
    api_func: update_guild_name,
    service: core,
    resp: Unit,
    req: UpdateGuildNameRequest,

    args {
        guild_id: u64 => guild_id: (|u| u);
        new_name: String => new_guild_name: (|n| n);
    }
}

client_api! {
    api_func: delete_guild,
    service: core,
    resp: Unit,
    req: DeleteGuildRequest,

    args {
        guild_id: u64 => guild_id: (|u| u);
    }
}

client_api_action! {
    api_func: join_guild,
    service: core,
    action: JoinGuild,

    args {
        invite_id: String => invite_id: (|n| n);
    }
}

client_api! {
    api_func: leave_guild,
    service: core,
    resp: Unit,
    req: LeaveGuildRequest,

    args {
        guild_id: u64 => guild_id: (|u| u);
    }
}

// GUILD
// INVITE

client_api_action! {
    api_func: create_invite,
    service: core,
    action: CreateInvite,

    args {
        name: String => name: (|n| n);
        guild_id: u64 => guild_id: (|g| g);
        possible_uses: i32 => possible_uses: (|p| p);
    }
}

client_api! {
    api_func: delete_invite,
    service: core,
    resp: Unit,
    req: DeleteInviteRequest,

    args {
        guild_id: u64 => guild_id: (|u| u);
        invite_id: String => invite_id: (|i| i);
    }
}

// INVITE
// CHANNEL

client_api_action! {
    api_func: create_channel,
    service: core,
    action: CreateChannel,

    args {
        name: String => channel_name: (|n| n);
        guild_id: u64 => guild_id: (|g| g);
        is_category: bool => is_category: (|i| i);
        previous_channel_id: Option<u64> => previous_id: Option::unwrap_or_default;
        next_channel_id: Option<u64> => next_id: Option::unwrap_or_default;
    }
}

client_api_action! {
    api_func: get_channel_messages,
    service: core,
    action: GetChannelMessages,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|c| c);
        before_message_id: Option<u64> => before_message: Option::unwrap_or_default;
    }
}

client_api! {
    api_func: update_channel_name,
    service: core,
    resp: Unit,
    req: UpdateChannelNameRequest,

    args {
        new_name: String => new_channel_name: (|n| n);
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|p| p);
    }
}

client_api! {
    api_func: update_channel_order,
    service: core,
    resp: Unit,
    req: UpdateChannelOrderRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        previous_channel_id: u64 => previous_id: (|p| p);
        next_channel_id: u64 => next_id: (|n| n);
    }
}

client_api! {
    api_func: delete_channel,
    service: core,
    resp: Unit,
    req: DeleteChannelRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|p| p);
    }
}

// CHANNEL
// EMOTE

client_api_action! {
    api_func: create_emote_pack,
    service: core,
    action: CreateEmotePack,

    args {
        name: String => pack_name: (|p| p);
    }
}

client_api_action! {
    api_func: get_emote_packs,
    service: core,
    action: GetEmotePacks,

    args { }
}

client_api_action! {
    api_func: get_emote_pack_emotes,
    service: core,
    action: GetEmotePackEmotes,

    args {
        pack_id: u64 => pack_id: (|g| g);
    }
}

client_api! {
    api_func: add_emote_to_pack,
    service: core,
    resp: Unit,
    req: AddEmoteToPackRequest,

    args {
        pack_id: u64 => pack_id: (|g| g);
        image_id: String => image_id: (|i| i);
        name: String => name: (|n| n);
    }
}

client_api! {
    api_func: delete_emote_from_pack,
    service: core,
    resp: Unit,
    req: DeleteEmoteFromPackRequest,

    args {
        pack_id: u64 => pack_id: (|g| g);
        image_id: String => image_id: (|i| i);
    }
}

client_api! {
    api_func: delete_emote_pack,
    service: core,
    resp: Unit,
    req: DeleteEmotePackRequest,

    args {
        pack_id: u64 => pack_id: (|g| g);
    }
}

client_api! {
    api_func: dequip_emote_pack,
    service: core,
    resp: Unit,
    req: DequipEmotePackRequest,

    args {
        pack_id: u64 => pack_id: (|g| g);
    }
}

// EMOTE
// MESSAGE

client_api_action! {
    api_func: get_message,
    service: core,
    action: GetMessage,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        message_id: u64 => message_id: (|g| g);
    }
}

client_api! {
    api_func: delete_message,
    service: core,
    resp: Unit,
    req: DeleteMessageRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        message_id: u64 => message_id: (|g| g);
    }
}

client_api_action! {
    api_func: send_message,
    service: core,
    action: SendMessage,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        in_reply_to: Option<u64> => in_reply_to: (|i: Option<u64>| i.unwrap_or_default());
        content: Option<String> => content: (|a: Option<String>| a.unwrap_or_default());
        embeds: Option<Vec<Embed>> => embeds: (|a: Option<Vec<Embed>>| a.unwrap_or_default());
        actions: Option<Vec<Action>> => actions: (|a: Option<Vec<Action>>| a.unwrap_or_default());
        attachments: Option<Vec<Uri>> => attachments: (|a: Option<Vec<Uri>>| a.unwrap_or_default().into_iter().map(|u| u.to_string()).collect());
        overrides: Option<Option<Override>> => overrides: (|a: Option<Option<Override>>| a.unwrap_or_default());
    }
}

pub async fn update_message(
    client: &Client,
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
    new_content: Option<String>,
    new_embeds: Option<Vec<Embed>>,
    new_actions: Option<Vec<Action>>,
    new_attachments: Option<Vec<Uri>>,
    new_overrides: Option<Option<Override>>,
) -> ClientResult<()> {
    let request = UpdateMessageRequest {
        guild_id,
        channel_id,
        message_id,
        update_content: new_content.is_some(),
        update_embeds: new_embeds.is_some(),
        update_actions: new_actions.is_some(),
        update_attachments: new_attachments.is_some(),
        update_overrides: new_overrides.is_some(),
        content: new_content.unwrap_or_default(),
        embeds: new_embeds.unwrap_or_default(),
        actions: new_actions.unwrap_or_default(),
        attachments: new_attachments
            .unwrap_or_default()
            .into_iter()
            .map(|url| url.to_string())
            .collect(),
        overrides: new_overrides.unwrap_or_default(),
    };

    client
        .core_lock()
        .update_message(request)
        .await
        .map(Response::into_inner)
        .map_err(Into::into)
}

// MESSAGE

client_api! {
    api_func: trigger_action,
    service: core,
    resp: Unit,
    req: TriggerActionRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        message_id: u64 => message_id: (|g| g);
        action_id: String => action_id: (|a| a);
        action_data: String => action_data: (|a| a);
    }
}

// PERMISSIONS

client_api_action! {
    api_func: get_permissions,
    service: core,
    action: GetPermissions,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        role_id: u64 => role_id: (|g| g);
    }
}

client_api_action! {
    api_func: query_has_permission,
    service: core,
    action: QueryPermissions,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        check_for: String => check_for: (|c| c);
        as_user_id: Option<u64> => r#as: (|a: Option<u64>| a.unwrap_or_default());
    }
}

client_api! {
    api_func: set_permissions,
    service: core,
    resp: Unit,
    req: SetPermissionsRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        channel_id: u64 => channel_id: (|g| g);
        role_id: u64 => role_id: (|g| g);
        permissions: Option<PermissionList> => perms: (|p| p);
    }
}

// PERMISSIONS
// ROLE

client_api_action! {
    api_func: get_guild_roles,
    service: core,
    action: GetGuildRoles,

    args {
        guild_id: u64 => guild_id: (|g| g);
    }
}

client_api_action! {
    api_func: add_guild_role,
    service: core,
    action: AddGuildRole,

    args {
        guild_id: u64 => guild_id: (|g| g);
        role: Role => role: Some;
    }
}

client_api! {
    api_func: delete_guild_role,
    service: core,
    resp: Unit,
    req: DeleteGuildRoleRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        role_id: u64 => role_id: (|g| g);
    }
}

client_api! {
    api_func: modify_guild_role,
    service: core,
    resp: Unit,
    req: ModifyGuildRoleRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        role: Role => role: Some;
        modify_name: bool => modify_name: (|m| m);
        modify_color: bool => modify_color: (|m| m);
        modify_hoist: bool => modify_hoist: (|m| m);
        modify_pingable: bool => modify_pingable: (|m| m);
    }
}

client_api_action! {
    api_func: move_role,
    service: core,
    action: MoveRole,

    args {
        guild_id: u64 => guild_id: (|g| g);
        role_id: u64 => role_id: (|g| g);
        before_role_id: u64 => before_id: (|g| g);
        after_role_id: u64 => after_id: (|g| g);
    }
}

client_api! {
    api_func: manage_user_roles,
    service: core,
    resp: Unit,
    req: ManageUserRolesRequest,

    args {
        guild_id: u64 => guild_id: (|g| g);
        user_id: u64 => user_id: (|g| g);
        give_role_ids: Vec<u64> => give_role_ids: (|g| g);
        take_role_ids: Vec<u64> => take_role_ids: (|g| g);
    }
}

client_api_action! {
    api_func: get_user_roles,
    service: core,
    action: GetUserRoles,

    args {
        guild_id: u64 => guild_id: (|g| g);
        user_id: u64 => user_id: (|g| g);
    }
}

// ROLE

pub async fn stream_events<
    S: futures::stream::Stream<Item = stream_events_request::Request> + Send + Sync + 'static,
>(
    client: &Client,
    requests: S,
) -> ClientResult<tonic::Streaming<Event>> {
    use futures::StreamExt;

    let requests = requests.map(|r| StreamEventsRequest { request: Some(r) });

    client
        .core_lock()
        .stream_events(requests)
        .await
        .map(Response::into_inner)
        .map_err(Into::into)
}
