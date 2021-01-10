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
    eprintln!("protocol");

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    eprintln!("OUT_DIR: {:?}", out_dir);
    #[cfg(feature = "gen_client")]
    {
        // Patch voice generated code (only for client)
        // We patch this because two methods with the same name are generated and prost
        // doesnt check this.
        let voice_gen_path = out_dir.join("protocol.voice.v1.rs");
        let voice_gen = std::fs::read_to_string(&voice_gen_path).unwrap();
        eprintln!("client patch");
        let patched_voice_gen = voice_gen.replace(
            "pub async fn connect<D>(dst: D)",
            "pub async fn _connect<D>(dst: D)",
        );
        std::fs::write(voice_gen_path, patched_voice_gen).unwrap();
        eprintln!("client patch");
    }
    // Patch chat message event
    // We patch these because of enum variant size differences, since they will be sent more than any other
    // event (its a realtime chat platform, so) this should help.
    let chat_gen_path = out_dir.join("protocol.chat.v1.rs");
    let chat_gen = std::fs::read_to_string(&chat_gen_path).unwrap();
    eprintln!("chat patch");
    let mut patched_chat_gen = chat_gen.replace(
        "SentMessage(MessageSent),",
        "SentMessage(Box<MessageSent>),",
    );
    patched_chat_gen = patched_chat_gen.replace(
        "EditedMessage(MessageUpdated),",
        "EditedMessage(Box<MessageUpdated>),",
    );
    std::fs::write(&chat_gen_path, patched_chat_gen).unwrap();
    eprintln!("chat patch");
}
