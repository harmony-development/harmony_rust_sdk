use crate::api::rest::*;

use prost::bytes::Bytes;
use reqwest::{multipart::*, Response};
use serde::Deserialize;

use super::{
    error::{ClientError, ClientResult},
    Client,
};

impl Client {
    /// Fetch a client's homeserver's about information.
    pub async fn about(&self) -> ClientResult<About> {
        let uri = format!("{}_harmony/about", self.homeserver_url());

        let request = self.data.http.get(uri.as_str()).build()?;
        tracing::debug!("Sending HTTP request: {:?}", request);

        let response = self.data.http.execute(request).await?;
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
        &self,
        filename: String,
        content_type: String,
        data: Vec<u8>,
    ) -> ClientResult<Response> {
        let (status, bytes) = self.auth_status_lock().clone();
        let token_bytes = if !status.is_authenticated() {
            return Err(ClientError::Unauthenticated);
        } else {
            bytes
        };

        // [ref:upload_path_create]
        let uri = format!("{}_harmony/media/upload", self.homeserver_url());

        let form = Form::new().part(
            "file",
            Part::bytes(data)
                .file_name(filename)
                .mime_str(&content_type)?,
        );

        let request = self
            .data
            .http
            .post(uri.as_str())
            .multipart(form)
            .header(
                http::header::AUTHORIZATION,
                http::HeaderValue::from_maybe_shared(token_bytes)
                    .expect("auth token must be UTF-8"),
            )
            .build()?;
        tracing::debug!("Sending HTTP request: {:?}", request);

        let response = self.data.http.execute(request).await?;
        tracing::debug!("Got HTTP response: {:?}", response);

        response.error_for_status().map_err(Into::into)
    }

    /// Downloads a file using a file ID.
    ///
    /// This endpoint does not require authentication.
    /// See [API documentation](https://github.com/harmony-development/protocol/blob/master/rest/rest.md#get-_harmonymediadownloadfile_id).
    pub async fn download(&self, file_id: impl Into<String>) -> ClientResult<Response> {
        let uri = file_id.into().make_download_url(self.homeserver_url());

        let request = self.data.http.get(uri.as_str()).build()?;
        tracing::debug!("Sending HTTP request: {:?}", request);

        let response = self.data.http.execute(request).await?;
        tracing::debug!("Got HTTP response: {:?}", response);

        response.error_for_status().map_err(Into::into)
    }

    /// Uploads a file and then extracts the file ID from the returned response.
    ///
    /// Also see [`Client::upload()`].
    pub async fn upload_extract_id(
        &self,
        filename: String,
        content_type: String,
        data: Vec<u8>,
    ) -> ClientResult<String> {
        #[derive(Debug, Deserialize)]
        struct FileId {
            id: String,
        }

        let resp = self.upload(filename, content_type, data).await?;
        let file_id: FileId = resp.json().await?;

        Ok(file_id.id)
    }

    /// Downloads a file then extracts file information from it.
    ///
    /// Also see [`Client::download()`].
    pub async fn download_extract_file(
        &self,
        file_id: impl Into<String>,
    ) -> ClientResult<DownloadedFile> {
        let resp = self.download(file_id).await?;

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
}

/// A downloaded file.
#[derive(Debug)]
#[non_exhaustive]
pub struct DownloadedFile {
    /// Data of the downloaded file.
    pub data: Bytes,
    /// Mimetype of the downloaded file.
    pub mimetype: String,
    /// File kind of the downloaded file.
    pub kind: FileKind,
    /// Name of the downloaded file.
    pub name: String,
}
