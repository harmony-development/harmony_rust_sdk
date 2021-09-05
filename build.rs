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
            "chat/v1/guilds.proto",
            "chat/v1/permissions.proto",
            "chat/v1/stream.proto",
        ];
        protos.append(&mut chat_protos);
    }

    #[cfg(feature = "gen_voice")]
    protos.push("voice/v1/voice.proto");

    #[cfg(feature = "gen_sync")]
    protos.push("sync/v1/sync.proto");

    #[cfg(feature = "gen_batch")]
    protos.push("batch/v1/batch.proto");

    #[cfg(feature = "gen_profile")]
    protos.push("profile/v1/profile.proto");

    #[cfg(feature = "gen_emote")]
    protos.push("emote/v1/emote.proto");

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
        patched_chat_gen = patched_chat_gen.replace(
            "pub embed: ::core::option::Option<super::Embed>,",
            "pub embed: ::core::option::Option<Box<super::Embed>>,",
        );
        std::fs::write(&chat_gen_path, patched_chat_gen).expect("Failed to read from chat service generated code, are you sure you have correct permissions?");
        write_permissions_rs(&out_dir);
    }
}

fn write_permissions_rs(out_dir: &std::path::Path) {
    use regex::Regex;
    use walkdir::WalkDir;

    let r = Regex::new(r#"option \(harmonytypes.v1.metadata\).requires_permission_node[ \n]+=[ \n]+"(?P<perm>.+)";"#).unwrap();

    let mut perms = String::new();

    let files = WalkDir::new("protocol")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in files {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".proto") {
            let text = std::fs::read_to_string(entry.path()).unwrap();
            for m in r.captures_iter(&text).flat_map(|c| c.name("perm")) {
                let perm = m.as_str();
                let const_name = perm
                    .replace(|c| ['.', '-'].contains(&c), "_")
                    .to_uppercase();
                let perm_const = format!(
                    "/// `{}` permission\npub const {}: &str = \"{}\";\n",
                    perm, const_name, perm
                );
                if !perms.contains(&perm_const) {
                    perms.push_str(&perm_const);
                }
            }
        }
    }

    let perms_path = out_dir.join("permissions.rs");
    std::fs::write(perms_path, perms).unwrap();
}
