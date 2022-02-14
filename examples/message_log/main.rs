//! Example showcasing a very simple message logging bot.
use harmony_rust_sdk::{
    api::{
        auth::{next_step_request::form_fields::Field, AuthStepResponse},
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
    let (exit_tx, mut exit_rx) = oneshot::channel::<()>();

    {
        // workaround because `ctrlc::set_handler` doesn't take an `FnOnce`
        // meaning it can't be used with `oneshot` sender since it's
        // `send` method consumes itself, which makes the closure `FnOnce`,
        // which doesn't implement `FnMut` since it can't be called more than once
        let mut exit_tx = Some(exit_tx);
        ctrlc::set_handler(move || {
            if let Some(exit_tx) = exit_tx.take() {
                // we ignore errors, since if the rx is dropped
                // it means we already exited
                let _ = exit_tx.send(());
            }
        })
        .expect("Can't set Ctrl-C handler");
    }

    let guild_invite = std::env::var("GUILD_INVITE");

    // Let's create our client first
    let client = Client::new(HOMESERVER.parse().unwrap(), None).await?;
    info!("Successfully created client.");

    // We try to login, if it fails we register (which also authenticates)
    client.begin_auth().await?;
    client.next_auth_step(AuthStepResponse::Initial).await?;
    client
        .next_auth_step(AuthStepResponse::choice("login"))
        .await?;
    let login_result = client
        .next_auth_step(AuthStepResponse::form(vec![
            Field::String(EMAIL.to_string()),
            Field::Bytes(PASSWORD.as_bytes().to_vec()),
        ]))
        .await;

    if login_result.map_or(false, |maybe_step| maybe_step.is_some()) {
        info!("Login failed, let's try registering.");
        client.prev_auth_step().await?;
        client
            .next_auth_step(AuthStepResponse::choice("register"))
            .await?;
        client
            .next_auth_step(AuthStepResponse::form(vec![
                Field::String(EMAIL.to_string()),
                Field::String(USERNAME.to_string()),
                Field::Bytes(PASSWORD.as_bytes().to_vec()),
            ]))
            .await?;
        info!("Successfully registered.");
    } else {
        info!("Successfully logon.");
    }

    // Change our bots status to online
    client
        .call(UpdateProfileRequest::default().with_new_user_status(UserStatus::Online))
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
            Ok(Some(event)) = socket.get_event() => {
                if let chat::Event::Chat(stream_event::Event::SentMessage(sent_message)) = event {
                    if let Some(message) = sent_message.message {
                        info!("Received new message: {:?}", message);
                        println!(
                            "Received new message with ID {}, from guild {} in channel {} sent by {}:\n{}",
                            sent_message.message_id,
                            sent_message.guild_id,
                            sent_message.channel_id,
                            message.author_id,
                            message.get_text().unwrap_or("<empty message>"),
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
        .call(UpdateProfileRequest::default().with_new_user_status(UserStatus::OfflineUnspecified))
        .await?;

    Ok(())
}
