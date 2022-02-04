use super::*;
use crate::client::error::ClientError;

use prost::bytes::Bytes;
use reqwest::{multipart::*, Response};
use serde::Deserialize;

pub use crate::api::rest::*;

/// Fetch a client's homeserver's about information.
pub async fn about(client: &Client) -> ClientResult<About> {
    let uri = format!("{}_harmony/about", client.homeserver_url());

    let request = client.data.http.get(uri.as_str()).build()?;
    tracing::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    tracing::debug!("Got HTTP response: {:?}", response);

    let response = response.error_for_status()?;
    let about: About = response.json().await?;

    Ok(about)
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
    let (status, bytes) = client.auth_status_lock().clone();
    let token_bytes = if !status.is_authenticated() {
        return Err(ClientError::Unauthenticated);
    } else {
        bytes
    };

    // [ref:upload_path_create]
    let uri = format!("{}_harmony/media/upload", client.homeserver_url());

    let form = Form::new().part(
        "file",
        Part::bytes(data)
            .file_name(filename)
            .mime_str(&content_type)?,
    );

    let request = client
        .data
        .http
        .post(uri.as_str())
        .multipart(form)
        .header(
            http::header::AUTHORIZATION,
            http::HeaderValue::from_maybe_shared(token_bytes).expect("auth token must be UTF-8"),
        )
        .build()?;
    tracing::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    tracing::debug!("Got HTTP response: {:?}", response);

    response.error_for_status().map_err(Into::into)
}

/// Downloads a file using a file ID.
///
/// This endpoint does not require authentication.
/// See [API documentation](https://github.com/harmony-development/protocol/blob/master/rest/rest.md#get-_harmonymediadownloadfile_id).
pub async fn download(client: &Client, file_id: impl Into<FileId>) -> ClientResult<Response> {
    let uri = file_id.into().make_download_url(client.homeserver_url());

    let request = client.data.http.get(uri.as_str()).build()?;
    tracing::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    tracing::debug!("Got HTTP response: {:?}", response);

    response.error_for_status().map_err(Into::into)
}

/// Uploads a file and then extracts the file ID from the returned response.
///
/// Also see [`upload()`].
pub async fn upload_extract_id(
    client: &Client,
    filename: String,
    content_type: String,
    data: Vec<u8>,
) -> ClientResult<String> {
    #[derive(Debug, Deserialize)]
    struct FileId {
        id: String,
    }

    let resp = upload(client, filename, content_type, data).await?;
    let file_id: FileId = resp.json().await?;

    Ok(file_id.id)
}

/// A downloaded file.
#[derive(Debug, Clone)]
pub struct DownloadedFile {
    data: Bytes,
    mimetype: String,
    kind: FileKind,
    name: String,
}

impl DownloadedFile {
    /// Get the file kind.
    pub fn kind(&self) -> FileKind {
        self.kind
    }

    /// Get the file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the mimetype.
    pub fn mimetype(&self) -> &str {
        &self.mimetype
    }

    /// Get the data.
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

/// Downloads a file then extracts file information from it.
///
/// Also see [`download()`].
pub async fn download_extract_file(
    client: &Client,
    file_id: impl Into<FileId>,
) -> ClientResult<DownloadedFile> {
    let resp = download(client, file_id).await?;

    let (name, mimetype, kind) = extract_file_info_from_download_response(resp.headers())
        .map(|(name, mimetype, kind)| {
            (
                name.to_owned(),
                mimetype
                    .to_str()
                    .map(ToOwned::to_owned)
                    .map_err(|_| ClientError::unexpected("Content-Type is not ASCII")),
                kind,
            )
        })
        .map_err(ClientError::unexpected)?;

    let data = resp.bytes().await?;

    Ok(DownloadedFile {
        data,
        mimetype: mimetype?,
        kind,
        name,
    })
}
