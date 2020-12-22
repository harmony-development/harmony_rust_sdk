//! Example showcasing a very simple echo bot.
use futures_util::StreamExt;
use harmony_rust_sdk::client::{api::core::*, Client, ClientResult};

const EMAIL: &str = "echo_bot@example.org";
const USERNAME: &str = "echo_bot";
const PASSWORD: &str = "very secret password!";
const HOMESERVER: &str = "http://127.0.0.1:2289";

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
    if let Err(_) = client.login(EMAIL, PASSWORD).await {
        log::info!("Login failed, let's try registering.");
        client.register(EMAIL, USERNAME, PASSWORD).await?;
        log::info!("Successfully registered.");
    } else {
        log::info!("Successfully logon.");
    }

    // Join the guild if invite is specified
    let guild_id = if let Ok(invite) = guild_invite {
        join_guild(&client, invite).await?.guild_id
    } else {
        std::fs::read_to_string(GUILD_ID_FILE)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    };
    std::fs::write(GUILD_ID_FILE, guild_id.to_string()).unwrap();
    log::info!("In guild: {}", guild_id);

    // Our bot's user id
    let self_id = client.session().unwrap().user_id;

    // Subscribe to guild events
    let mut subscription = client
        .subscribe_events(vec![guild_id], false, false)
        .await?;

    // Poll events
    // If stream is finished (should not happen) or an error occurs, abort processing
    while let Some(Ok(received_event)) = subscription.next().await {
        if let event::Event::SentMessage(sent_message) = received_event {
            if let Some(message) = sent_message.message {
                // Dont sent message if we sent it
                if message.author_id != self_id {
                    log::info!("Echoing message: {}", message.message_id);
                    send_message(
                        &client,
                        guild_id,
                        message.channel_id,
                        Some(message.in_reply_to),
                        Some(message.content),
                        Some(message.embeds),
                        Some(message.actions),
                        None, // can't copy attachments because we don't get the url back?
                        Some(message.overrides),
                    )
                    .await?;
                }
            }
        }
    }
    log::error!("An error occured while getting events from the server. Aborting!");

    Ok(())
}
