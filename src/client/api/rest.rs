use crate::client::{api::Hmc, *};

use http::uri::PathAndQuery;
use reqwest::{multipart::*, Response};

/// A "file id", which can either be a HMC URL or a plain ID string.
#[derive(Debug, Clone)]
pub enum FileId {
    /// A HMC describing where the file is.
    Hmc(Hmc),
    /// A plain ID. When you use this for a request, the `Client`s homeserver will be used.
    Id(String),
}

/// Uploads a file to the homeserver.
///
/// This endpoint requires authentication.
/// See [API documentation](https://github.com/harmony-development/protocol/blob/master/rest/rest.md#post-_harmonymediaupload).
pub async fn upload(
    client: &Client,
    filename: String,
    content_type: String,
    data: Vec<u8>,
) -> ClientResult<Response> {
    let session_token = if let AuthStatus::Complete(session) = client.auth_status() {
        session.session_token
    } else {
        return Err(ClientError::Unauthenticated);
    };

    let api = PathAndQuery::from_static("/_harmony/media/upload");
    // This unwrap is safe, since our client's homeserver url is valid, and the path we create is also checked at compile time.
    let uri = Uri::from_parts(assign::assign!(client.homeserver_url().clone().into_parts(), { path_and_query: Some(api), })).unwrap();

    let form = Form::new().part("file", Part::bytes(data));

    let request = client
        .data
        .http
        .post(uri.to_string().as_str())
        .multipart(form)
        .header("Authorization", session_token)
        .query(&[("filename", &filename), ("contentType", &content_type)])
        .build()?;
    log::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    log::debug!("Got HTTP response: {:?}", response);

    response.error_for_status().map_err(Into::into)
}

/// Downloads a file using a file ID.
///
/// This endpoint does not require authentication.
/// See [API documentation](https://github.com/harmony-development/protocol/blob/master/rest/rest.md#get-_harmonymediadownloadfile_id).
pub async fn download(client: &Client, file_id: FileId) -> ClientResult<Response> {
    let (scheme, server, id) = match file_id {
        FileId::Hmc(hmc) => ("https", hmc.server().to_string(), hmc.id().to_string()),
        FileId::Id(id) => {
            let url = client.homeserver_url();
            (
                url.scheme_str().unwrap(), // Safe since we can't create a client without a scheme
                url.authority().unwrap().to_string(), // Safe since we can't create a client without an authority (it cant connect)
                id,
            )
        }
    };

    let uri = Uri::builder()
        .scheme(scheme)
        .authority(server.as_str())
        .path_and_query(format!("/_harmony/media/download/{}", id))
        .build()?;

    let request = client.data.http.get(uri.to_string().as_str()).build()?;
    log::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    log::debug!("Got HTTP response: {:?}", response);

    response.error_for_status().map_err(Into::into)
}
