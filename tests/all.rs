use harmony_rust_sdk::{client::Client, exports::http::Uri};

// Make sure legato is running on `http://127.0.0.1:2289` before running this test.
#[tokio::test]
async fn all() {
    env_logger::builder().is_test(true).init();

    let email = String::from("example@example.org");
    let username = String::from("example");
    let password = String::from("123456789");

    let client = Client::new("http://127.0.0.1".parse::<Uri>().unwrap(), None)
        .await
        .unwrap();

    let response = client.login(email.clone(), password.clone()).await;

    log::info!("{:?}", response);

    let response = client
        .register(email.clone(), username.clone(), password.clone())
        .await;

    log::info!("{:?}", response);
}
