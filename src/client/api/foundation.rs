use crate::{
    api::foundation::{login_request::*, *},
    client::{Client, ClientResult},
    client_api,
};
use tonic::{Request, Response};

client_api! {
    args: {
        email: String,
        username: String,
        password: String,
    },
    response: Session,
    request: RegisterRequest {
        email, username,
        password: password.into_bytes(),
    },
    api_func: register,
    service: foundation,
}

client_api! {
    args: {
        email: String,
        password: String,
    },
    response: Session,
    request: LoginRequest {
        login: Some(Login::Local(Local {
            email,
            password: password.into_bytes(),
        })),
    },
    api_func: login,
    service: foundation,
}
