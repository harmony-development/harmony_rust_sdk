use std::{ops::Not, path::PathBuf};

use harmony_build::{Builder, Protocol, Result};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(
        std::env::var_os("OUT_DIR")
            .ok_or("Failed to get OUT_DIR! Something must be horribly wrong.")?,
    );

    let protocol_path = std::env::var_os("HARMONY_PROTOCOL_PATH")
        .map_or_else(|| PathBuf::from("protocol"), PathBuf::from);

    #[rustfmt::skip]
    let stable_svcs = [
        #[cfg(feature = "gen_harmonytypes")] "harmonytypes.v1",
        #[cfg(feature = "gen_auth")] "auth.v1",
        #[cfg(feature = "gen_mediaproxy")] "mediaproxy.v1",
        #[cfg(feature = "gen_chat")] "chat.v1",
        #[cfg(feature = "gen_sync")] "sync.v1",
        #[cfg(feature = "gen_batch")] "batch.v1",
        #[cfg(feature = "gen_profile")] "profile.v1",
        #[cfg(feature = "gen_emote")] "emote.v1",
    ];

    #[rustfmt::skip]
    let staging_svcs = [
        #[cfg(feature = "staging_gen_voice")] "voice.v1",
        #[cfg(feature = "staging_gen_bots")] "bots.v1",
    ];

    let all_services = stable_svcs
        .into_iter()
        .chain(staging_svcs.into_iter())
        .collect::<Vec<_>>();

    let protocol = Protocol::from_path(protocol_path, &stable_svcs, &staging_svcs)?;

    let mut builder = harmony_build::Builder::new();

    if cfg!(feature = "_client_common") {
        let add_impl_call_req = |builder: Builder, service: &str| {
            builder.modify_hrpc_config(|cfg| {
                cfg.type_attribute(
                    format!(".protocol.{}", service),
                    format!("#[harmony_derive::impl_call_req({})]", service),
                )
            })
        };

        let for_svcs = all_services
            .iter()
            .filter(|svc| matches!(**svc, "harmonytypes.v1" | "sync.v1").not());

        for service in for_svcs {
            builder = add_impl_call_req(builder, service);
        }
    }

    builder = builder.modify_hrpc_config(|cfg| {
        cfg.type_attribute(".", "#[harmony_derive::self_builder_with_new]")
    });
    builder = builder.modify_prost_config(|mut cfg| {
        cfg.bytes(&[".protocol.batch.v1"]);
        cfg
    });
    if cfg!(feature = "all_permissions") {
        builder = builder.write_permissions(true);
    }
    if cfg!(feature = "rkyv") {
        for service in all_services.iter().filter(|a| "batch.v1".ne(**a)) {
            builder = builder.modify_hrpc_config(|cfg| {
                cfg.type_attribute(
                    format!(".protocol.{}", service),
                    "#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]",
                )
            });
        }
    }

    if protocol.protos().is_empty().not() {
        builder.generate(protocol, out_dir)?;
    }

    Ok(())
}
