use harmony_rust_sdk::{
    api::chat::{GetEmotePacksRequest, GetGuildListRequest},
    client::{
        api::{
            auth::*,
            chat::{channel::*, message::*, profile::*, *},
            *,
        },
        error::*,
        *,
    },
};
use hrpc::url::Url;
use rest::FileId;
use tracing::info;

const EMAIL: &str = "rust_sdk_test@example.org";
const PASSWORD: &str = "123456789Ab";

const TEST_SERVER: &str = "https://chat.harmonyapp.io:2289";
const TEST_SERVER_NAME_RES: &str = "https://chat.harmonyapp.io";
const TEST_GUILD: u64 = 2699074975217745925;
const TEST_CHANNEL: u64 = 2700365654061481989;

const FILE_DATA: &str = "They're waiting for you Gordon, in the test chamber.";
const FILENAME: &str = "test_chamber.txt";
const CONTENT_TYPE: &str = "text/plain";
const FILE_ID: &str = "403cb46c-49cf-4ae1-b876-f38eb26accb0";
const EXTERNAL_URL: &str =
    "https://cdn.discordapp.com/avatars/363103389992747019/34ee306c324137ffdef785b1537672cd.jpg";

const INSTANT_VIEW_URL: &str = "https://duckduckgo.com/";

#[tokio::test(flavor = "current_thread")]
async fn main() -> ClientResult<()> {
    env_logger::init();

    {
        info!("Testing name resolution...");
        Client::new(TEST_SERVER_NAME_RES.parse().unwrap(), None).await?;
        info!("Name resolution works!");
    }

    info!("Testing client connection...");
    let client = Client::new(TEST_SERVER.parse().unwrap(), None).await?;
    info!("Created client");

    info!("Testing auth...");
    client.begin_auth().await?;
    client.next_auth_step(AuthStepResponse::Initial).await?;
    client
        .next_auth_step(AuthStepResponse::login_choice())
        .await?;
    client
        .next_auth_step(AuthStepResponse::login_form(EMAIL, PASSWORD))
        .await?;
    assert_eq!(client.auth_status().is_authenticated(), true);
    info!("Logged in");

    info!("Testing profile update...");
    profile::profile_update(
        &client,
        ProfileUpdate::default().new_status(harmonytypes::UserStatus::OnlineUnspecified),
    )
    .await?;
    info!("Updated profile");

    let response = guild::preview_guild(&client, invite::InviteId::new("harmony").unwrap()).await?;
    info!("Preview guild response: {:?}", response);
    assert_eq!(response.name.as_str(), "Harmony Development");

    let response = guild::get_guild_list(&client, GetGuildListRequest {}).await?;
    info!("Get guild list response: {:?}", response);

    // let response = permissions::get_guild_roles(&client, TEST_GUILD).await?;
    // info!("Get guild roles response: {:?}", response);

    let members_response = guild::get_guild_members(&client, GuildId::new(TEST_GUILD)).await?;
    info!("Get guild members response: {:?}", members_response);

    let response = profile::get_user(
        &client,
        UserId::new(
            *members_response
                .members
                .first()
                .expect("expected at least one user in guild"),
        ),
    )
    .await?;
    info!("Get user response: {:?}", response);

    let response = profile::get_user_bulk(&client, members_response.members).await?;
    info!("Get user bulk response: {:?}", response);

    let response = emote::get_emote_packs(&client, GetEmotePacksRequest {}).await?;
    info!("Get emote packs response: {:?}", response);

    let response = channel::get_guild_channels(&client, GuildId::new(TEST_GUILD)).await?;
    info!("Get guild channels response: {:?}", response);

    typing(&client, Typing::new(TEST_GUILD, TEST_CHANNEL)).await?;
    info!("Notified the server that we are typing");

    let current_time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let msg = format!("test at {}", current_time);
    message::send_message(
        &client,
        SendMessage::new(TEST_GUILD, TEST_CHANNEL, msg.clone()),
    )
    .await?;
    info!("Sent a test message");

    let response =
        channel::get_channel_messages(&client, GetChannelMessages::new(TEST_GUILD, TEST_CHANNEL))
            .await?;
    info!("Get channel messages response: {:?}", response);
    let our_msg = response.messages.first().unwrap();
    assert_eq!(our_msg.content, msg.as_str());

    let instant_view =
        mediaproxy::instant_view(&client, INSTANT_VIEW_URL.parse::<Url>().unwrap()).await?;
    info!("Instant view response: {:?}", instant_view);
    assert_eq!(&instant_view.metadata.unwrap().url, INSTANT_VIEW_URL);

    let can_instant_view =
        mediaproxy::can_instant_view(&client, INSTANT_VIEW_URL.parse::<Url>().unwrap()).await?;
    info!("Can instant view response: {:?}", can_instant_view);

    let fetch_link_metadata =
        mediaproxy::fetch_link_metadata(&client, INSTANT_VIEW_URL.parse::<Url>().unwrap()).await?;
    info!("Fetch link metadata response: {:?}", fetch_link_metadata);

    let response = rest::upload(
        &client,
        FILENAME.to_string(),
        CONTENT_TYPE.to_string(),
        FILE_DATA.as_bytes().to_vec(),
    )
    .await?;
    info!("Upload file response: {:?}", response);

    let file_id = response.text().await?;
    info!("Uploaded file, returned ID: {}", file_id);

    let response = rest::download(&client, rest::FileId::Id(FILE_ID.to_string())).await?;
    info!("Download file response: {:?}", response);

    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap() // The server should send this header. If not, it's an error.
        .to_str()
        .unwrap() // Content type should be an ascii string, since its a mimetype.
        .to_string();

    assert_eq!(response.text().await?.as_str(), FILE_DATA);
    assert_eq!(content_type.as_str(), CONTENT_TYPE);

    let response = rest::download(
        &client,
        rest::FileId::Hmc(Hmc::new(
            TEST_SERVER
                .parse::<Url>()
                .unwrap()
                .host()
                .unwrap() // must have authority
                .to_owned(),
            FILE_ID.to_string(),
        )),
    )
    .await?;
    info!("Download file response: {:?}", response);

    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap() // The server should send this header. If not, it's an error.
        .to_str()
        .unwrap() // Content type should be an ascii string, since its a mimetype.
        .to_string();

    assert_eq!(response.text().await?.as_str(), FILE_DATA);
    assert_eq!(content_type.as_str(), CONTENT_TYPE);

    let downloaded_file = rest::download_extract_file(&client, client.make_hmc(FILE_ID)).await?;
    assert_eq!(
        downloaded_file
            .data()
            .clone()
            .into_iter()
            .map(char::from)
            .collect::<String>()
            .as_str(),
        FILE_DATA
    );
    assert_eq!(downloaded_file.mimetype(), CONTENT_TYPE);
    assert_eq!(downloaded_file.name(), FILENAME);

    let external_file =
        rest::download(&client, FileId::External(EXTERNAL_URL.parse().unwrap())).await?;
    let _ = external_file.bytes().await?;

    profile::profile_update(
        &client,
        ProfileUpdate::default().new_status(harmonytypes::UserStatus::Offline),
    )
    .await?;

    Ok(())
}
