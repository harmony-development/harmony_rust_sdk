use harmony_api::{
    exports::prost,
    foundation::{self, Session},
};

use std::sync::{Arc, Mutex};

use http::Uri;
use hyper::{client::HttpConnector, Body, Client as HyperClient, Request, Response};
use prost::Message;

type Connector = hyper_rustls::HttpsConnector<HttpConnector>;

#[derive(Debug)]
struct ClientData {
    homeserver_url: Uri,
    session: Mutex<Option<Session>>,
    http_client: HyperClient<Connector>,
}

#[derive(Clone, Debug)]
pub struct Client {
    data: Arc<ClientData>,
}

impl Client {
    pub fn new(homeserver_url: Uri, session: Option<Session>) -> Self {
        let data = ClientData {
            homeserver_url,
            session: Mutex::new(session),
            http_client: HyperClient::builder().build(Connector::new()),
        };

        Self {
            data: Arc::new(data),
        }
    }

    pub fn session(&self) -> Option<Session> {
        self.data
            .session
            .lock()
            .expect("session mutex was poisoned")
            .clone()
    }

    pub async fn login(&self, login: foundation::login_request::Login) -> Response<Body> {
        let mut buf = Vec::new();
        let login_request = foundation::LoginRequest { login: Some(login) };
        login_request.encode(&mut buf).unwrap();

        let request = Request::builder()
            .method("POST")
            .uri(&self.data.homeserver_url.to_string())
            .body(Body::from(buf))
            .unwrap();

        self.data.http_client.request(request).await.unwrap()
    }
}
