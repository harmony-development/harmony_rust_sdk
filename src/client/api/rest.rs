use crate::client::{api::Hmc, *};

use http::uri::PathAndQuery;
use prost::bytes::Bytes;
use reqwest::{multipart::*, Response};
use serde::Deserialize;

/// A "file id", which can either be a HMC URL or a plain ID string.
#[derive(Debug, Clone)]
pub enum FileId {
    /// A HMC describing where the file is.
    Hmc(Hmc),
    /// A plain ID. When you use this for a request, the `Client`s homeserver will be used.
    Id(String),
    /// An external URI. This MUST be an image according to the protocol.
    External(Uri),
}

impl From<Hmc> for FileId {
    fn from(hmc: Hmc) -> Self {
        Self::Hmc(hmc)
    }
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
pub async fn download(client: &Client, file_id: impl Into<FileId>) -> ClientResult<Response> {
    let (scheme, server, id) = match file_id.into() {
        FileId::Hmc(hmc) => ("https", hmc.server().to_string(), hmc.id().to_string()),
        FileId::Id(id) => {
            let url = client.homeserver_url();
            (
                url.scheme_str().unwrap(), // Safe since we can't create a client without a scheme
                url.authority().unwrap().to_string(), // Safe since we can't create a client without an authority (it cant connect)
                id,
            )
        }
        FileId::External(uri) => {
            let url = client.homeserver_url();
            (
                url.scheme_str().unwrap(), // Safe since we can't create a client without a scheme
                url.authority().unwrap().to_string(), // Safe since we can't create a client without an authority (it cant connect)
                urlencoding::encode(&uri.to_string()),
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

/// Kind of the file downloaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
    Attachment,
    Inline,
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

    let mimetype = resp
        .headers()
        .get("Content-Type")
        .ok_or_else(|| {
            ClientError::unexpected("server did not respond with `Content-Type` header")
        })?
        .to_str()
        .map_err(|e| {
            ClientError::unexpected(format!(
                "server responded with non ASCII content type: {}",
                e
            ))
        })?
        .to_owned();

    let mut split = resp
        .headers()
        .get("Content-Disposition")
        .ok_or_else(|| {
            ClientError::unexpected("server did not respond with `Content-Disposition` header")
        })?
        .to_str()
        .map_err(|e| {
            ClientError::unexpected(format!(
                "server responded with non ASCII content disposition: {}",
                e
            ))
        })?
        .split(';');
    let kind = match split
        .next()
        .ok_or_else(|| ClientError::unexpected("server did not respond with file kind"))?
    {
        "attachment" => FileKind::Attachment,
        "inline" => FileKind::Inline,
        other => {
            return Err(ClientError::unexpected(format!(
                "server responded with invalid file kind: {}",
                other
            )))
        }
    };
    let no_name = || ClientError::unexpected("server did not respond with a file name");
    let name = split
        .next()
        .ok_or_else(no_name)?
        .split('=')
        .nth(1)
        .ok_or_else(no_name)?
        .trim_matches('"')
        .to_owned();
    let data = resp.bytes().await?;

    Ok(DownloadedFile {
        data,
        kind,
        name,
        mimetype,
    })
}
