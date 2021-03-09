(function() {var implementors = {};
implementors["harmony_rust_sdk"] = [{"text":"impl Send for GuildListEntry","synthetic":true,"types":[]},{"text":"impl Send for Invite","synthetic":true,"types":[]},{"text":"impl Send for Channel","synthetic":true,"types":[]},{"text":"impl Send for EmotePack","synthetic":true,"types":[]},{"text":"impl Send for Emote","synthetic":true,"types":[]},{"text":"impl Send for Mode","synthetic":true,"types":[]},{"text":"impl Send for SubscribeToGuild","synthetic":true,"types":[]},{"text":"impl Send for SubscribeToActions","synthetic":true,"types":[]},{"text":"impl Send for SubscribeToHomeserverEvents","synthetic":true,"types":[]},{"text":"impl Send for Request","synthetic":true,"types":[]},{"text":"impl Send for MessageSent","synthetic":true,"types":[]},{"text":"impl Send for MessageUpdated","synthetic":true,"types":[]},{"text":"impl Send for MessageDeleted","synthetic":true,"types":[]},{"text":"impl Send for ChannelCreated","synthetic":true,"types":[]},{"text":"impl Send for ChannelUpdated","synthetic":true,"types":[]},{"text":"impl Send for ChannelDeleted","synthetic":true,"types":[]},{"text":"impl Send for GuildUpdated","synthetic":true,"types":[]},{"text":"impl Send for GuildDeleted","synthetic":true,"types":[]},{"text":"impl Send for MemberJoined","synthetic":true,"types":[]},{"text":"impl Send for MemberLeft","synthetic":true,"types":[]},{"text":"impl Send for GuildAddedToList","synthetic":true,"types":[]},{"text":"impl Send for GuildRemovedFromList","synthetic":true,"types":[]},{"text":"impl Send for ActionPerformed","synthetic":true,"types":[]},{"text":"impl Send for RoleMoved","synthetic":true,"types":[]},{"text":"impl Send for ProfileUpdated","synthetic":true,"types":[]},{"text":"impl Send for Typing","synthetic":true,"types":[]},{"text":"impl Send for LeaveReason","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for DmInvite","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for ChatServiceClient","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for ChatServiceServer&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Send for GetUserRequest","synthetic":true,"types":[]},{"text":"impl Send for GetUserResponse","synthetic":true,"types":[]},{"text":"impl Send for GetUserMetadataRequest","synthetic":true,"types":[]},{"text":"impl Send for GetUserMetadataResponse","synthetic":true,"types":[]},{"text":"impl Send for ProfileUpdateRequest","synthetic":true,"types":[]},{"text":"impl Send for CreateGuildRequest","synthetic":true,"types":[]},{"text":"impl Send for CreateGuildResponse","synthetic":true,"types":[]},{"text":"impl Send for CreateInviteRequest","synthetic":true,"types":[]},{"text":"impl Send for CreateInviteResponse","synthetic":true,"types":[]},{"text":"impl Send for GetGuildListRequest","synthetic":true,"types":[]},{"text":"impl Send for GetGuildListResponse","synthetic":true,"types":[]},{"text":"impl Send for GetGuildRequest","synthetic":true,"types":[]},{"text":"impl Send for GetGuildResponse","synthetic":true,"types":[]},{"text":"impl Send for GetGuildInvitesRequest","synthetic":true,"types":[]},{"text":"impl Send for GetGuildInvitesResponse","synthetic":true,"types":[]},{"text":"impl Send for GetGuildMembersRequest","synthetic":true,"types":[]},{"text":"impl Send for GetGuildMembersResponse","synthetic":true,"types":[]},{"text":"impl Send for UpdateGuildInformationRequest","synthetic":true,"types":[]},{"text":"impl Send for DeleteGuildRequest","synthetic":true,"types":[]},{"text":"impl Send for DeleteInviteRequest","synthetic":true,"types":[]},{"text":"impl Send for JoinGuildRequest","synthetic":true,"types":[]},{"text":"impl Send for JoinGuildResponse","synthetic":true,"types":[]},{"text":"impl Send for PreviewGuildRequest","synthetic":true,"types":[]},{"text":"impl Send for PreviewGuildResponse","synthetic":true,"types":[]},{"text":"impl Send for LeaveGuildRequest","synthetic":true,"types":[]},{"text":"impl Send for AddGuildToGuildListRequest","synthetic":true,"types":[]},{"text":"impl Send for AddGuildToGuildListResponse","synthetic":true,"types":[]},{"text":"impl Send for RemoveGuildFromGuildListRequest","synthetic":true,"types":[]},{"text":"impl Send for RemoveGuildFromGuildListResponse","synthetic":true,"types":[]},{"text":"impl Send for BanUserRequest","synthetic":true,"types":[]},{"text":"impl Send for KickUserRequest","synthetic":true,"types":[]},{"text":"impl Send for UnbanUserRequest","synthetic":true,"types":[]},{"text":"impl Send for CreateChannelRequest","synthetic":true,"types":[]},{"text":"impl Send for CreateChannelResponse","synthetic":true,"types":[]},{"text":"impl Send for GetGuildChannelsRequest","synthetic":true,"types":[]},{"text":"impl Send for GetGuildChannelsResponse","synthetic":true,"types":[]},{"text":"impl Send for UpdateChannelInformationRequest","synthetic":true,"types":[]},{"text":"impl Send for UpdateChannelOrderRequest","synthetic":true,"types":[]},{"text":"impl Send for DeleteChannelRequest","synthetic":true,"types":[]},{"text":"impl Send for TypingRequest","synthetic":true,"types":[]},{"text":"impl Send for GetChannelMessagesRequest","synthetic":true,"types":[]},{"text":"impl Send for GetChannelMessagesResponse","synthetic":true,"types":[]},{"text":"impl Send for GetMessageRequest","synthetic":true,"types":[]},{"text":"impl Send for GetMessageResponse","synthetic":true,"types":[]},{"text":"impl Send for UpdateMessageRequest","synthetic":true,"types":[]},{"text":"impl Send for DeleteMessageRequest","synthetic":true,"types":[]},{"text":"impl Send for TriggerActionRequest","synthetic":true,"types":[]},{"text":"impl Send for SendMessageRequest","synthetic":true,"types":[]},{"text":"impl Send for SendMessageResponse","synthetic":true,"types":[]},{"text":"impl Send for CreateEmotePackRequest","synthetic":true,"types":[]},{"text":"impl Send for CreateEmotePackResponse","synthetic":true,"types":[]},{"text":"impl Send for GetEmotePacksRequest","synthetic":true,"types":[]},{"text":"impl Send for GetEmotePacksResponse","synthetic":true,"types":[]},{"text":"impl Send for GetEmotePackEmotesRequest","synthetic":true,"types":[]},{"text":"impl Send for GetEmotePackEmotesResponse","synthetic":true,"types":[]},{"text":"impl Send for AddEmoteToPackRequest","synthetic":true,"types":[]},{"text":"impl Send for DeleteEmoteFromPackRequest","synthetic":true,"types":[]},{"text":"impl Send for DeleteEmotePackRequest","synthetic":true,"types":[]},{"text":"impl Send for DequipEmotePackRequest","synthetic":true,"types":[]},{"text":"impl Send for QueryPermissionsRequest","synthetic":true,"types":[]},{"text":"impl Send for QueryPermissionsResponse","synthetic":true,"types":[]},{"text":"impl Send for Permission","synthetic":true,"types":[]},{"text":"impl Send for PermissionList","synthetic":true,"types":[]},{"text":"impl Send for SetPermissionsRequest","synthetic":true,"types":[]},{"text":"impl Send for GetPermissionsRequest","synthetic":true,"types":[]},{"text":"impl Send for GetPermissionsResponse","synthetic":true,"types":[]},{"text":"impl Send for Role","synthetic":true,"types":[]},{"text":"impl Send for MoveRoleRequest","synthetic":true,"types":[]},{"text":"impl Send for MoveRoleResponse","synthetic":true,"types":[]},{"text":"impl Send for GetGuildRolesRequest","synthetic":true,"types":[]},{"text":"impl Send for GetGuildRolesResponse","synthetic":true,"types":[]},{"text":"impl Send for AddGuildRoleRequest","synthetic":true,"types":[]},{"text":"impl Send for AddGuildRoleResponse","synthetic":true,"types":[]},{"text":"impl Send for DeleteGuildRoleRequest","synthetic":true,"types":[]},{"text":"impl Send for ModifyGuildRoleRequest","synthetic":true,"types":[]},{"text":"impl Send for ManageUserRolesRequest","synthetic":true,"types":[]},{"text":"impl Send for GetUserRolesRequest","synthetic":true,"types":[]},{"text":"impl Send for GetUserRolesResponse","synthetic":true,"types":[]},{"text":"impl Send for StreamEventsRequest","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for SyncRequest","synthetic":true,"types":[]},{"text":"impl Send for SyncEvent","synthetic":true,"types":[]},{"text":"impl Send for Place","synthetic":true,"types":[]},{"text":"impl Send for InviteId","synthetic":true,"types":[]},{"text":"impl Send for FormField","synthetic":true,"types":[]},{"text":"impl Send for Choice","synthetic":true,"types":[]},{"text":"impl Send for Form","synthetic":true,"types":[]},{"text":"impl Send for Waiting","synthetic":true,"types":[]},{"text":"impl Send for Step","synthetic":true,"types":[]},{"text":"impl Send for Field","synthetic":true,"types":[]},{"text":"impl Send for Choice","synthetic":true,"types":[]},{"text":"impl Send for FormFields","synthetic":true,"types":[]},{"text":"impl Send for Form","synthetic":true,"types":[]},{"text":"impl Send for Step","synthetic":true,"types":[]},{"text":"impl Send for AuthServiceClient","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for AuthServiceServer&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Send for BeginAuthResponse","synthetic":true,"types":[]},{"text":"impl Send for Session","synthetic":true,"types":[]},{"text":"impl Send for AuthStep","synthetic":true,"types":[]},{"text":"impl Send for NextStepRequest","synthetic":true,"types":[]},{"text":"impl Send for StepBackRequest","synthetic":true,"types":[]},{"text":"impl Send for StreamStepsRequest","synthetic":true,"types":[]},{"text":"impl Send for FederateRequest","synthetic":true,"types":[]},{"text":"impl Send for FederateReply","synthetic":true,"types":[]},{"text":"impl Send for KeyReply","synthetic":true,"types":[]},{"text":"impl Send for LoginFederatedRequest","synthetic":true,"types":[]},{"text":"impl Send for Reason","synthetic":true,"types":[]},{"text":"impl Send for HarmonyMethodMetadata","synthetic":true,"types":[]},{"text":"impl Send for Override","synthetic":true,"types":[]},{"text":"impl Send for Action","synthetic":true,"types":[]},{"text":"impl Send for EmbedHeading","synthetic":true,"types":[]},{"text":"impl Send for EmbedField","synthetic":true,"types":[]},{"text":"impl Send for Embed","synthetic":true,"types":[]},{"text":"impl Send for Attachment","synthetic":true,"types":[]},{"text":"impl Send for Metadata","synthetic":true,"types":[]},{"text":"impl Send for Message","synthetic":true,"types":[]},{"text":"impl Send for Error","synthetic":true,"types":[]},{"text":"impl Send for UserStatus","synthetic":true,"types":[]},{"text":"impl Send for ActionType","synthetic":true,"types":[]},{"text":"impl Send for ActionPresentation","synthetic":true,"types":[]},{"text":"impl Send for FieldPresentation","synthetic":true,"types":[]},{"text":"impl Send for Data","synthetic":true,"types":[]},{"text":"impl Send for MediaProxyServiceClient","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for MediaProxyServiceServer&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Send for SiteMetadata","synthetic":true,"types":[]},{"text":"impl Send for MediaMetadata","synthetic":true,"types":[]},{"text":"impl Send for FetchLinkMetadataRequest","synthetic":true,"types":[]},{"text":"impl Send for FetchLinkMetadataResponse","synthetic":true,"types":[]},{"text":"impl Send for InstantViewRequest","synthetic":true,"types":[]},{"text":"impl Send for InstantViewResponse","synthetic":true,"types":[]},{"text":"impl Send for CanInstantViewResponse","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for VoiceServiceClient","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for VoiceServiceServer&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl Send for Signal","synthetic":true,"types":[]},{"text":"impl Send for ConnectRequest","synthetic":true,"types":[]},{"text":"impl Send for ConnectResponse","synthetic":true,"types":[]},{"text":"impl Send for StreamStateRequest","synthetic":true,"types":[]},{"text":"impl Send for HmcParseError","synthetic":true,"types":[]},{"text":"impl Send for Hmc","synthetic":true,"types":[]},{"text":"impl Send for AuthStepResponse","synthetic":true,"types":[]},{"text":"impl Send for AuthResponse","synthetic":true,"types":[]},{"text":"impl Send for AuthId","synthetic":true,"types":[]},{"text":"impl Send for GetChannelMessages","synthetic":true,"types":[]},{"text":"impl Send for CreateChannel","synthetic":true,"types":[]},{"text":"impl Send for DeleteChannel","synthetic":true,"types":[]},{"text":"impl Send for UpdateChannelInformation","synthetic":true,"types":[]},{"text":"impl Send for UpdateChannelOrder","synthetic":true,"types":[]},{"text":"impl Send for PackId","synthetic":true,"types":[]},{"text":"impl Send for CreateEmotePack","synthetic":true,"types":[]},{"text":"impl Send for AddEmoteToPack","synthetic":true,"types":[]},{"text":"impl Send for DeleteEmoteFromPack","synthetic":true,"types":[]},{"text":"impl Send for CreateGuild","synthetic":true,"types":[]},{"text":"impl Send for GuildList","synthetic":true,"types":[]},{"text":"impl Send for UpdateGuildInformation","synthetic":true,"types":[]},{"text":"impl Send for CreateInvite","synthetic":true,"types":[]},{"text":"impl Send for DeleteInvite","synthetic":true,"types":[]},{"text":"impl Send for SendMessage","synthetic":true,"types":[]},{"text":"impl Send for UpdateMessage","synthetic":true,"types":[]},{"text":"impl Send for GetPermissions","synthetic":true,"types":[]},{"text":"impl Send for QueryPermissions","synthetic":true,"types":[]},{"text":"impl Send for SetPermissions","synthetic":true,"types":[]},{"text":"impl Send for AddGuildRole","synthetic":true,"types":[]},{"text":"impl Send for DeleteGuildRole","synthetic":true,"types":[]},{"text":"impl Send for ModifyGuildRole","synthetic":true,"types":[]},{"text":"impl Send for MoveRole","synthetic":true,"types":[]},{"text":"impl Send for ManageUserRoles","synthetic":true,"types":[]},{"text":"impl Send for GetUserRoles","synthetic":true,"types":[]},{"text":"impl Send for AppId","synthetic":true,"types":[]},{"text":"impl Send for ProfileUpdate","synthetic":true,"types":[]},{"text":"impl Send for EventSource","synthetic":true,"types":[]},{"text":"impl Send for MessageLocation","synthetic":true,"types":[]},{"text":"impl Send for GuildId","synthetic":true,"types":[]},{"text":"impl Send for UserId","synthetic":true,"types":[]},{"text":"impl Send for TriggerAction","synthetic":true,"types":[]},{"text":"impl Send for Typing","synthetic":true,"types":[]},{"text":"impl Send for FileId","synthetic":true,"types":[]},{"text":"impl Send for InvalidFileId","synthetic":true,"types":[]},{"text":"impl Send for FileIds","synthetic":true,"types":[]},{"text":"impl Send for FileKind","synthetic":true,"types":[]},{"text":"impl Send for DownloadedFile","synthetic":true,"types":[]},{"text":"impl Send for ClientError","synthetic":true,"types":[]},{"text":"impl Send for AuthStatus","synthetic":true,"types":[]},{"text":"impl Send for Client","synthetic":true,"types":[]},{"text":"impl Send for EventsSocket","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()