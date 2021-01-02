//! Example showcasing a very simple message logging bot.
use futures_util::StreamExt;
use harmony_rust_sdk::client::{
    api::{auth::*, chat::*, *},
    Client, ClientResult,
};

const EMAIL: &str = "message_log_bot@example.org";
const USERNAME: &str = "message_log_bot";
const PASSWORD: &str = "very secret password!";
const HOMESERVER: &str = "https://127.0.0.1:2289";

const GUILD_ID_FILE: &str = "guild_id";

// Be sure to add the bot to your server once it registers and give it the necessary permissions.
#[tokio::main]
async fn main() -> ClientResult<()> {
    // Init logging
    env_logger::init();

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
        None,
        Some(UserStatus::OnlineUnspecified),
        None,
        Some(true),
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
    let mut event_stream = client
        .subscribe_events(vec![EventSource::Guild(guild_id)])
        .await?;

    // Poll events
    loop {
        if let Some(Ok(received_event)) = event_stream.next().await {
            if let event::Event::SentMessage(sent_message) = received_event {
                if let Some(message) = sent_message.message {
                    log::info!("Received new message: {:?}", message);
                    println!(
                        "Received new message with ID {}, from guild {} in channel {} sent by {}:\n {}",
                        message.message_id,
                        message.guild_id,
                        message.channel_id,
                        message.author_id,
                        message.content
                    );
                }
            }
        }
    }
}
