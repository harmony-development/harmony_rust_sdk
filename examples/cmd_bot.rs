//! Example showcasing a very simple echo bot.
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, UNIX_EPOCH},
};

use harmony_rust_sdk::{
    api::chat::event,
    client::{
        api::{
            auth::AuthStepResponse,
            chat::{
                guild,
                invite::InviteId,
                message::{self, SendMessage},
                profile::{self, ProfileUpdate},
                EventSource, UserId,
            },
            harmonytypes::UserStatus,
        },
        error::ClientResult,
        Client,
    },
};

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
    log::info!("Successfully created client.");

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
        log::info!("Login failed, let's try registering.");
        client.prev_auth_step().await?;
        client
            .next_auth_step(AuthStepResponse::register_choice())
            .await?;
        client
            .next_auth_step(AuthStepResponse::register_form(EMAIL, USERNAME, PASSWORD))
            .await?;
        log::info!("Successfully registered.");
    } else {
        log::info!("Successfully logon.");
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
    log::info!("In guild: {}", guild_id);

    // Subscribe to guild events
    let mut socket = client
        .subscribe_events(vec![EventSource::Guild(guild_id)])
        .await?;

    let start = std::time::Instant::now();

    // Poll events
    loop {
        if DID_CTRLC.load(Ordering::Relaxed) {
            break;
        }
        if let Some(Ok(event::Event::SentMessage(sent_message))) = socket.get_event().await {
            if let Some(message) = sent_message.message {
                log::info!("got message: {}", message.content);
                if message.content.starts_with("r!") {
                    let mut parts = message.content[2..].split_whitespace();
                    if let Some(cmd) = parts.next() {
                        match cmd {
                            "ping" => {
                                let created_at = {
                                    let tmp = message.created_at.unwrap_or_default();
                                    Duration::new(tmp.seconds as u64, tmp.nanos as u32)
                                };
                                let arrive_duration =
                                    (UNIX_EPOCH.elapsed().unwrap() - created_at).as_secs_f32();

                                message::send_message(
                                    &client,
                                    SendMessage::new(
                                        guild_id,
                                        message.channel_id,
                                        format!("Pong! Took {} secs.", arrive_duration),
                                    ),
                                )
                                .await?;
                            }
                            "hello" => {
                                let user_profile =
                                    profile::get_user(&client, UserId::new(message.author_id))
                                        .await?;

                                message::send_message(
                                    &client,
                                    SendMessage::new(
                                        guild_id,
                                        message.channel_id,
                                        format!("Hello, {}!", user_profile.user_name),
                                    ),
                                )
                                .await?;
                            }
                            "uptime" => {
                                message::send_message(
                                    &client,
                                    SendMessage::new(
                                        guild_id,
                                        message.channel_id,
                                        format!(
                                            "Been running for {} seconds.",
                                            start.elapsed().as_secs()
                                        ),
                                    ),
                                )
                                .await?;
                            }
                            _ => {}
                        }
                    }
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
