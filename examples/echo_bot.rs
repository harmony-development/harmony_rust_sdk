//! Example showcasing a very simple echo bot.
use harmony_rust_sdk::client::{api::core::*, Client, ClientResult};

const EMAIL: &str = "echo_bot@example.org";
const USERNAME: &str = "echo_bot";
const PASSWORD: &str = "very secret password!";
const HOMESERVER: &str = "http://127.0.0.1:2289";

const GUILD_ID_FILE: &str = "guild_id";
const CHANNEL_ID_FILE: &str = "channel_id";

// Be sure to add the bot to your server once it registers and give it the necessary permissions.
#[tokio::main]
async fn main() -> ClientResult<()> {
    // Init logging
    env_logger::init();

    let guild_invite = std::env::var("GUILD_INVITE");
    let channel_to_listen = std::env::var("CHANNEL_TO_LISTEN").map(|e| e.parse::<u64>().unwrap());

    // Let's create our client first
    let client = Client::new(HOMESERVER.parse().unwrap(), None).await?;
    log::info!("Successfully created client.");

    // We try to login, if it fails we register (which also authenticates)
    if let Err(_) = client.login(EMAIL.to_string(), PASSWORD.to_string()).await {
        log::info!("Login failed, let's try registering.");
        client
            .register(
                EMAIL.to_string(),
                USERNAME.to_string(),
                PASSWORD.to_string(),
            )
            .await?;
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

    let channel_to_listen = if let Ok(channel_id) = channel_to_listen {
        channel_id
    } else {
        std::fs::read_to_string(CHANNEL_ID_FILE)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    };
    std::fs::write(CHANNEL_ID_FILE, channel_to_listen.to_string()).unwrap();
    log::info!("Using channel: {}", channel_to_listen);

    let self_id = client.session().unwrap().user_id;
    // Message ID that we currently read up to (and including)
    let mut read_up_to_message_id = None;
    while let Ok(messages_response) =
        get_channel_messages(&client, guild_id, channel_to_listen, read_up_to_message_id).await
    {
        let read_up_to = messages_response.messages.last().map(|msg| msg.message_id);
        // Lets send back messages
        for message in messages_response
            .messages
            .into_iter()
            .filter(|msg| msg.author_id != self_id)
        {
            log::info!("Echoing message: {}", message.message_id);
            send_message(
                &client,
                guild_id,
                channel_to_listen,
                Some(message.in_reply_to),
                Some(message.content),
                Some(message.embeds),
                Some(message.actions),
                None, // can't copy attachments because we don't get the url back?
                Some(message.overrides),
            )
            .await?;
        }

        if read_up_to.is_some() {
            read_up_to_message_id = read_up_to;
            log::info!("Read up to message: {}", read_up_to_message_id.unwrap());
        }
    }

    Ok(())
}
