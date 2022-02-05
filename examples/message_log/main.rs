//! Example showcasing a very simple message logging bot.
use std::sync::atomic::{AtomicBool, Ordering};

use harmony_rust_sdk::{
    api::{
        auth::AuthStepResponse,
        chat::{self, stream_event, EventSource, JoinGuildRequest},
        profile::{UpdateProfileRequest, UserStatus},
    },
    client::{error::ClientResult, Client},
};
use tokio::sync::oneshot;
use tracing::info;
use tracing_subscriber::EnvFilter;

const EMAIL: &str = "rust_sdk_test@example.org";
const USERNAME: &str = "rust_sdk_test";
const PASSWORD: &str = "123456789Ab";
const HOMESERVER: &str = "https://chat.harmonyapp.io:2289";

const GUILD_ID_FILE: &str = "guild_id";

#[tokio::main(flavor = "current_thread")]
async fn main() -> ClientResult<()> {
    // Init logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info")),
        )
        .init();

    // Create channels for ctrl-c shutdown
    let (exit_tx, exit_rx) = oneshot::channel::<()>();

    ctrlc::set_handler(move || {
        // we ignore errors, since if the rx is dropped
        // it means we already exited
        let _ = exit_tx.send(());
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
        .call(
            UpdateProfile::default()
                .with_new_status(UserStatus::Online)
                .with_new_is_bot(true),
        )
        .await?;

    // Join the guild if invite is specified
    let guild_id = if let Ok(invite) = guild_invite {
        client.call(JoinGuildRequest::new(invite)).await?.guild_id
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

    let mut socket = client.subscribe_events(false).await?;
    socket.add_source(EventSource::Guild(guild_id)).await?;

    // Start the main event loop
    loop {
        tokio::select! {
            // Handle events from our socket
            Some(event) = socket.get_event() => {
                if let chat::Event::Chat(stream_event::Event::SentMessage(sent_message)) = event {
                    if let Some(message) = sent_message.message {
                        info!("Received new message: {:?}", message);
                        println!(
                            "Received new message with ID {}, from guild {} in channel {} sent by {}:\n{}",
                            sent_message.message_id,
                            sent_message.guild_id,
                            sent_message.channel_id,
                            message.author_id,
                            message.text().unwrap_or("<empty message>"),
                        );
                    }
                }
            }
            // We will break from the loop when ctrl-c is pressed
            _ = &mut exit_rx => break,
            else => break,
        }
    }

    // Change our bots status back to offline
    client
        .call(UpdateProfile::default().with_new_status(UserStatus::OfflineUnspecified))
        .await?;

    Ok(())
}
