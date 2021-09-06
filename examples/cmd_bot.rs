//! Example showcasing a very simple echo bot.
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, UNIX_EPOCH},
};

use harmony_rust_sdk::{
    api::chat::{self, stream_event},
    client::{
        api::{
            auth::AuthStepResponse,
            chat::{
                invite::InviteId,
                message::{MessageExt, SendMessage},
                EventSource, UserId,
            },
            profile::{UpdateProfile, UserStatus},
        },
        error::ClientResult,
        Client,
    },
};

use tracing::info;
use tracing_subscriber::EnvFilter;

const EMAIL: &str = "rust_sdk_test@example.org";
const USERNAME: &str = "rust_sdk_test";
const PASSWORD: &str = "123456789Ab";
const HOMESERVER: &str = "https://chat.harmonyapp.io:2289";

const GUILD_ID_FILE: &str = "guild_id";

static DID_CTRLC: AtomicBool = AtomicBool::new(false);

#[tokio::main(flavor = "current_thread")]
async fn main() -> ClientResult<()> {
    // Init logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info")),
        )
        .init();

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
    client
        .profile()
        .await
        .update_profile(
            UpdateProfile::default()
                .with_new_status(UserStatus::Online)
                .with_new_is_bot(true),
        )
        .await?;

    // Join the guild if invite is specified
    let guild_id = if let Ok(invite) = guild_invite {
        client
            .chat()
            .await
            .join_guild(InviteId::new(invite).unwrap())
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

    let start = std::time::Instant::now();

    client
        .event_loop(
            vec![EventSource::Guild(guild_id)],
            move |client, event| async move {
                if DID_CTRLC.load(Ordering::Relaxed) {
                    return Ok(true);
                }
                if let chat::Event::Chat(stream_event::Event::SentMessage(sent_message)) = event {
                    if let Some(message) = sent_message.message {
                        let text_content = message.text().unwrap_or("<empty message>");
                        info!("got message: {}", text_content);
                        if let Some(mut parts) = text_content
                            .strip_prefix("r!")
                            .map(|c| c.split_whitespace())
                        {
                            if let Some(cmd) = parts.next() {
                                let reply = match cmd {
                                    "ping" => {
                                        let created_at = {
                                            let tmp = message.created_at.unwrap_or_default();
                                            Duration::new(tmp.seconds as u64, tmp.nanos as u32)
                                        };
                                        let arrive_duration = (UNIX_EPOCH.elapsed().unwrap()
                                            - created_at)
                                            .as_secs_f32();

                                        format!("Pong! Took {} secs.", arrive_duration)
                                    }
                                    "hello" => {
                                        let user_profile = client
                                            .profile()
                                            .await
                                            .get_profile(UserId::new(message.author_id))
                                            .await?;

                                        format!(
                                            "Hello, {}!",
                                            user_profile
                                                .profile
                                                .as_ref()
                                                .map_or("unknown", |p| p.user_name.as_str())
                                        )
                                    }
                                    "uptime" => {
                                        format!(
                                            "Been running for {} seconds.",
                                            start.elapsed().as_secs()
                                        )
                                    }
                                    _ => "No such command.".to_string(),
                                };
                                client
                                    .call(
                                        SendMessage::new(guild_id, sent_message.channel_id)
                                            .with_in_reply_to(sent_message.message_id)
                                            .text(reply),
                                    )
                                    .await?;
                            }
                        }
                    }
                }
                Ok(false)
            },
        )
        .await?;

    // Change our bots status back to offline
    client
        .profile()
        .await
        .update_profile(UpdateProfile::default().with_new_status(UserStatus::OfflineUnspecified))
        .await?;

    Ok(())
}
