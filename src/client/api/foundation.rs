use crate::{
    api::foundation::{login_request::*, *},
    client::{Client, ClientResult},
};

/// Send a login request to the server and return the session.
pub async fn login(client: &Client, email: String, password: String) -> ClientResult<Session> {
    let request = LoginRequest {
        login: Some(Login::Local(Local {
            email,
            password: password.into_bytes(),
        })),
    };

    log::debug!("Sending login request {:?}", request);
    client
        .foundation_lock()
        .login(request)
        .await
        .map(|e| e.into_inner())
        .map_err(Into::into)
}

/// Send a register request to the server and return the session.
pub async fn register(
    client: &Client,
    email: String,
    username: String,
    password: String,
) -> ClientResult<Session> {
    let request = RegisterRequest {
        email,
        username,
        password: password.into_bytes(),
    };

    log::debug!("Sending register request {:?}", request);
    client
        .foundation_lock()
        .register(request)
        .await
        .map(|e| e.into_inner())
        .map_err(Into::into)
}
