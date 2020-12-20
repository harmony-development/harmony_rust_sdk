use harmony_api::foundation::login_request::{Local, Login};
use harmony_client::Client;
use http::Uri;

#[tokio::test]
async fn login() {
    let client = Client::new("https://127.0.0.1".parse::<Uri>().unwrap(), None);

    let response = client
        .login(Login::Local(Local {
            email: String::from("example@example.org"),
            password: "123456789".as_bytes().to_vec(),
        }))
        .await;
}
