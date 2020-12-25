fn main() {
    #[allow(unused_mut)]
    let mut builder = tonic_build::configure();
    #[cfg(not(feature = "gen_server"))]
    {
        builder = builder.build_server(false);
    }
    #[cfg(not(feature = "gen_client"))]
    {
        builder = builder.build_client(false);
    }
    builder
        .compile(
            &[
                "harmonytypes/v1/types.proto",
                "auth/v1/auth.proto",
                "chat/v1/chat.proto",
                "chat/v1/messages.proto",
                "chat/v1/channels.proto",
                "chat/v1/emotes.proto",
                "chat/v1/guilds.proto",
                "chat/v1/permissions.proto",
                "chat/v1/profile.proto",
                "chat/v1/streaming.proto",
            ],
            &["protocol"],
        )
        .unwrap();
}
