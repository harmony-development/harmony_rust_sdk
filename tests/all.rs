use harmony_rust_sdk::client::{
    api::{auth::*, chat::*},
    *,
};
use http::Uri;

const EMAIL: &str = "rust_sdk_test@example.org";
const PASSWORD: &str = "123456789Ab";

const TEST_SERVER: &str = "chat.harmonyapp.io:2289";
const TEST_SERVER_NAME_RES: &str = "chat.harmonyapp.io";
const TEST_GUILD: u64 = 2699074975217745925;
const TEST_CHANNEL: u64 = 2700365654061481989;

const FILE_DATA: &str = "They're waiting for you Gordon, in the test chamber.";
const FILENAME: &str = "test_chamber.txt";
const CONTENT_TYPE: &str = "text/plain";
const FILE_ID: &str = "403cb46c-49cf-4ae1-b876-f38eb26accb0";

#[tokio::test]
async fn main() -> ClientResult<()> {
    env_logger::init();

    {
        log::info!("Testing name resolution...");
        Client::new(Uri::from_static(TEST_SERVER_NAME_RES), None).await?;
    }

    log::info!("Testing client connection...");
    let client = Client::new(Uri::from_static(TEST_SERVER), None).await?;
    log::info!("Created client");

    log::info!("Testing auth...");
    client.begin_auth().await?;
    client.next_auth_step(AuthStepResponse::Initial).await?;
    client
        .next_auth_step(AuthStepResponse::login_choice())
        .await?;
    client
        .next_auth_step(AuthStepResponse::login_form(EMAIL, PASSWORD))
        .await?;
    assert_eq!(client.auth_status().is_authenticated(), true);
    log::info!("Logged in");

    log::info!("Testing profile update...");
    api::chat::profile::profile_update(
        &client,
        None,
        Some(api::UserStatus::OnlineUnspecified),
        None,
        Some(true),
    )
    .await?;
    log::info!("Updated profile");

    let response = guild::preview_guild(&client, InviteId::new("harmony").unwrap()).await?;
    log::info!("Preview guild response: {:?}", response);

    let response = api::chat::guild::get_guild_list(&client).await?;
    log::info!("Get guild list response: {:?}", response);

    // let response = api::chat::permissions::get_guild_roles(&client, TEST_GUILD).await?;
    // log::info!("Get guild roles response: {:?}", response);

    let response = api::chat::profile::get_guild_members(&client, TEST_GUILD).await?;
    log::info!("Get guild members response: {:?}", response);

    let response = api::chat::emote::get_emote_packs(&client).await?;
    log::info!("Get emote packs response: {:?}", response);

    let response = api::chat::channel::get_guild_channels(&client, TEST_GUILD).await?;
    log::info!("Get guild channels response: {:?}", response);

    let response =
        api::chat::channel::get_channel_messages(&client, TEST_GUILD, TEST_CHANNEL, None).await?;
    log::info!("Get channel messages response: {:?}", response);

    typing(&client, TEST_GUILD, TEST_CHANNEL).await?;
    log::info!("Notified the server that we are typing");

    message::send_message(
        &client,
        TEST_GUILD,
        TEST_CHANNEL,
        None,
        None,
        Some("test".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .await?;
    log::info!("Sent a test message");

    let instant_view =
        api::mediaproxy::instant_view(&client, Uri::from_static("https://duckduckgo.com")).await?;
    log::info!("Instant view response: {:?}", instant_view);

    let can_instant_view =
        api::mediaproxy::can_instant_view(&client, Uri::from_static("https://duckduckgo.com"))
            .await?;
    log::info!("Can instant view response: {:?}", can_instant_view);

    let _event_stream = client
        .subscribe_events(vec![EventSource::Homeserver])
        .await?;

    let response = api::rest::upload(
        &client,
        FILENAME.to_string(),
        CONTENT_TYPE.to_string(),
        FILE_DATA.as_bytes().to_vec(),
    )
    .await?;
    log::info!("Upload file response: {:?}", response);

    let file_id = response.text().await?;
    log::info!("Uploaded file, returned ID: {}", file_id);

    let response = api::rest::download(&client, api::rest::FileId::Id(FILE_ID.to_string())).await?;
    log::info!("Download file response: {:?}", response);

    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap() // The server should send this header. If not, it's an error.
        .to_str()
        .unwrap() // Content type should be an ascii string, since its a mimetype.
        .to_string();

    assert_eq!(response.text().await?.as_str(), FILE_DATA);
    assert_eq!(content_type.as_str(), CONTENT_TYPE);

    let response = api::rest::download(
        &client,
        api::rest::FileId::Hmc(api::Hmc::new(
            Uri::from_static(TEST_SERVER)
                .authority()
                .unwrap() // must have authority
                .clone(),
            FILE_ID.to_string(),
        )),
    )
    .await?;
    log::info!("Download file response: {:?}", response);

    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap() // The server should send this header. If not, it's an error.
        .to_str()
        .unwrap() // Content type should be an ascii string, since its a mimetype.
        .to_string();

    assert_eq!(response.text().await?.as_str(), FILE_DATA);
    assert_eq!(content_type.as_str(), CONTENT_TYPE);

    Ok(())
}
