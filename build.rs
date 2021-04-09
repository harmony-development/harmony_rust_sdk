fn main() {
    #[allow(unused_mut)]
    let mut builder = hrpc_build::configure();

    let mut protos = Vec::with_capacity(12);

    #[cfg(feature = "gen_harmonytypes")]
    protos.push("harmonytypes/v1/types.proto");

    #[cfg(feature = "gen_auth")]
    protos.push("auth/v1/auth.proto");

    #[cfg(feature = "gen_mediaproxy")]
    protos.push("mediaproxy/v1/mediaproxy.proto");

    #[cfg(feature = "gen_chat")]
    {
        let mut chat_protos = vec![
            "chat/v1/chat.proto",
            "chat/v1/messages.proto",
            "chat/v1/channels.proto",
            "chat/v1/emotes.proto",
            "chat/v1/guilds.proto",
            "chat/v1/permissions.proto",
            "chat/v1/profile.proto",
            "chat/v1/streaming.proto",
        ];
        protos.append(&mut chat_protos);
    }

    #[cfg(feature = "gen_voice")]
    protos.push("voice/v1/voice.proto");

    let protocol_path =
        std::env::var("HARMONY_PROTOCOL_PATH").unwrap_or_else(|_| "protocol".to_string());
    builder.compile(&protos, &[protocol_path.as_str()]).expect(
        "\nProtobuf code generation failed! Are you sure you have `protoc` installed?\nIf so, please also set the PROTOC and PROTOC_INCLUDE as mentioned in the README.\nError",
    );

    #[cfg(any(feature = "gen_chat", feature = "gen_harmonytypes"))]
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR")
            .expect("Failed to get OUT_DIR! Something must be horribly wrong."),
    );

    #[cfg(feature = "gen_chat")]
    {
        // Patch chat message event
        // We patch these because of enum variant size differences, since they will be sent more than any other
        // event (its a realtime chat platform, so) this should help.
        let chat_gen_path = out_dir.join("protocol.chat.v1.rs");
        let chat_gen = std::fs::read_to_string(&chat_gen_path).expect("Failed to read from chat service generated code, are you sure you have correct permissions?");
        let mut patched_chat_gen = chat_gen.replace(
            "SentMessage(MessageSent),",
            "SentMessage(Box<MessageSent>),",
        );
        patched_chat_gen = patched_chat_gen.replace(
            "EditedMessage(MessageUpdated),",
            "EditedMessage(Box<MessageUpdated>),",
        );
        std::fs::write(&chat_gen_path, patched_chat_gen).expect("Failed to read from chat service generated code, are you sure you have correct permissions?");
    }

    #[cfg(feature = "gen_harmonytypes")]
    {
        let type_gen_path = out_dir.join("protocol.harmonytypes.v1.rs");
        let type_gen = std::fs::read_to_string(&type_gen_path).expect("Failed to read from chat service generated code, are you sure you have correct permissions?");
        let patched_type_gen = type_gen.replace(
            "pub embeds: ::core::option::Option<Embed>,",
            "pub embeds: ::core::option::Option<Box<Embed>>,",
        );
        std::fs::write(&type_gen_path, patched_type_gen).expect("Failed to read from chat service generated code, are you sure you have correct permissions?");
    }
}
