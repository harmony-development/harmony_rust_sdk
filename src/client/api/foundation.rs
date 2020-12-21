use crate::{
    api::foundation::{login_request::*, *},
    client::{Client, ClientResult},
    client_api,
};
use tonic::{Request, Response};

client_api! {
    api_func: register,
    service: foundation,
    resp: Session,
    req: RegisterRequest,

    args {
        email: String => email: (|e| e);
        username: String => username: (|u| u);
        password: String => password: (
            |p: String| p.into_bytes()
        );
    }
}

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
        .map(Response::into_inner)
        .map_err(Into::into)
}
