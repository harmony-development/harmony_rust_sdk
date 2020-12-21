use harmony_rust_sdk::client::{Client, ClientResult};

const EMAIL: &str = "example@example.org";
const USERNAME: &str = "example";
const PASSWORD: &str = "123456789";

// Make sure legato is running on `http://127.0.0.1:2289` before running this test.
#[tokio::test]
async fn all() -> ClientResult<()> {
    env_logger::builder().is_test(true).init();

    let client = Client::new("http://127.0.0.1".parse().unwrap(), None).await?;

    if client.login(EMAIL, PASSWORD).await.is_err() {
        client.register(EMAIL, USERNAME, PASSWORD).await?;
    }

    // TODO: do more API calls

    Ok(())
}
