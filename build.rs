fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

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
                "mediaproxy/v1/mediaproxy.proto",
                "chat/v1/chat.proto",
                "chat/v1/messages.proto",
                "chat/v1/channels.proto",
                "chat/v1/emotes.proto",
                "chat/v1/guilds.proto",
                "chat/v1/permissions.proto",
                "chat/v1/profile.proto",
                "chat/v1/streaming.proto",
                "voice/v1/voice.proto",
            ],
            &["protocol"],
        )
        .unwrap();
    #[cfg(feature = "gen_client")]
    {
        // Patch voice generated code
        let voice_gen_path = out_dir.join("protocol.voice.v1.rs");
        let voice_gen = std::fs::read_to_string(&voice_gen_path).unwrap();
        let patched_voice_gen = voice_gen.replace(
            "pub async fn connect<D>(dst: D)",
            "pub async fn _connect<D>(dst: D)",
        );
        std::fs::write(voice_gen_path, patched_voice_gen).unwrap();
    }
}
