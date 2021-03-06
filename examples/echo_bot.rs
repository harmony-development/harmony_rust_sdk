//! Example showcasing a very simple echo bot.
use std::{
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use harmony_rust_sdk::{
    api::chat::event,
    client::{
        api::{
            auth::AuthStepResponse,
            chat::{
                guild,
                invite::InviteId,
                message::{self, SendMessage, SendMessageSelfBuilder},
                profile::{self, ProfileUpdate},
                EventSource,
            },
            harmonytypes::UserStatus,
            rest::FileId,
        },
        error::ClientResult,
        Client,
    },
};

use tracing::info;

const EMAIL: &str = "rust_sdk_test@example.org";
const USERNAME: &str = "rust_sdk_test";
const PASSWORD: &str = "123456789Ab";
const HOMESERVER: &str = "https://chat.harmonyapp.io:2289";

const GUILD_ID_FILE: &str = "guild_id";

static DID_CTRLC: AtomicBool = AtomicBool::new(false);

#[tokio::main(flavor = "current_thread")]
async fn main() -> ClientResult<()> {
    // Init logging
    env_logger::init();

    ctrlc::set_handler(|| {
        DID_CTRLC.store(true, Ordering::Relaxed);
    })
    .expect("Can't set Ctrl-C handler");

    let guild_invite = std::env::var("GUILD_INVITE");

    // Let's create our client first
    let client = Client::new(HOMESERVER.parse().unwrap(), None).await?;
    info!("Successfully created client.");

    // We try to login, if it fails we register (which also authenticates)
    client.begin_auth().await?;
    client.next_auth_step(AuthStepResponse::Initial).await?;
    client
        .next_auth_step(AuthStepResponse::login_choice())
        .await?;
    let login_result = client
        .next_auth_step(AuthStepResponse::login_form(EMAIL, PASSWORD))
        .await;

    if login_result.map_or(false, |maybe_step| maybe_step.is_some()) {
        info!("Login failed, let's try registering.");
        client.prev_auth_step().await?;
        client
            .next_auth_step(AuthStepResponse::register_choice())
            .await?;
        client
            .next_auth_step(AuthStepResponse::register_form(EMAIL, USERNAME, PASSWORD))
            .await?;
        info!("Successfully registered.");
    } else {
        info!("Successfully logon.");
    }

    // Change our bots status to online and make sure its marked as a bot
    profile::profile_update(
        &client,
        ProfileUpdate::default()
            .new_status(UserStatus::OnlineUnspecified)
            .new_is_bot(true),
    )
    .await?;

    // Join the guild if invite is specified
    let guild_id = if let Ok(invite) = guild_invite {
        guild::join_guild(&client, InviteId::new(invite).unwrap())
            .await?
            .guild_id
    } else {
        tokio::fs::read_to_string(GUILD_ID_FILE)
            .await
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    };
    tokio::fs::write(GUILD_ID_FILE, guild_id.to_string())
        .await
        .unwrap();
    info!("In guild: {}", guild_id);

    // Our bot's user id
    let self_id = client.auth_status().session().unwrap().user_id;

    // Subscribe to guild events
    let mut socket = client
        .subscribe_events(vec![EventSource::Guild(guild_id)])
        .await?;

    // Poll events
    loop {
        if DID_CTRLC.load(Ordering::Relaxed) {
            break;
        }
        if let Some(Ok(event::Event::SentMessage(sent_message))) = socket.get_event().await {
            if let Some(message) = sent_message.message {
                // Dont sent message if we sent it
                if message.author_id != self_id {
                    info!("Echoing message: {}", message.message_id);

                    let attachments: Vec<FileId> = message
                        .attachments
                        .into_iter()
                        .flat_map(|a| FileId::from_str(&a.id))
                        .collect();

                    let send_message =
                        SendMessage::new(guild_id, message.channel_id, message.content)
                            .in_reply_to(message.in_reply_to)
                            .embeds(message.embeds)
                            .attachments(attachments)
                            .actions(message.actions)
                            .overrides(message.overrides)
                            .metadata(message.metadata);

                    message::send_message(&client, send_message).await?;
                }
            }
        }
    }

    // Change our bots status back to offline
    profile::profile_update(
        &client,
        ProfileUpdate::default().new_status(UserStatus::Offline),
    )
    .await?;

    Ok(())
}
