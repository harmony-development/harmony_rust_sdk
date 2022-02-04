use std::{convert::TryInto, error::Error as StdError, str::FromStr};

use super::Hmc;
use derive_more::{Display, From, Into, IntoIterator};
use derive_new::new;
use http::{HeaderValue, Uri};
use serde::{Deserialize, Serialize};

/// Kind of the file downloaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
    Attachment,
    Inline,
}

/// A "file id", which can be a HMC URL, an external URL or a plain ID string.
#[derive(Debug, Clone, Display, PartialEq, Eq, Hash)]
pub enum FileId {
    /// A HMC describing where the file is.
    Hmc(Hmc),
    /// A plain ID. When you use this in a request, the `Client`s homeserver will be used.
    Id(String),
    /// An external URL.
    External(Uri),
}

impl FileId {
    /// Create a download URL for download a file, from a file ID.
    ///
    /// The `homeserver_url` argument is used for [`FileId::Id`] and [`FileId::External`]
    /// variants, where a homeserver URL is needed for proxying the request through it.
    ///
    /// The returned string will always be a valid [`Uri`].
    pub fn make_download_url(&self, homeserver_url: &Uri) -> String {
        const ENDPOINT: &str = "/_harmony/media/download/";

        match self {
            FileId::Hmc(hmc) => format!(
                "https://{}:{}{}{}",
                hmc.server(),
                hmc.port(),
                ENDPOINT,
                hmc.id()
            )
            .parse()
            .unwrap(),
            FileId::Id(id) => format!(
                "{}{}{}",
                homeserver_url,
                ENDPOINT.trim_start_matches('/'),
                id
            ),
            FileId::External(uri) => format!(
                "{}{}{}",
                homeserver_url,
                ENDPOINT.trim_start_matches('/'),
                urlencoding::encode(uri.to_string().as_str())
            ),
        }
    }
}

impl From<Uri> for FileId {
    fn from(uri: Uri) -> Self {
        Self::External(uri)
    }
}

impl From<Hmc> for FileId {
    fn from(hmc: Hmc) -> Self {
        Self::Hmc(hmc)
    }
}

impl AsRef<FileId> for FileId {
    fn as_ref(&self) -> &FileId {
        self
    }
}

impl From<FileId> for String {
    fn from(o: FileId) -> String {
        match o {
            FileId::Hmc(hmc) => hmc.into(),
            FileId::Id(id) => id,
            FileId::External(url) => url.to_string(),
        }
    }
}

/// Error that may be produced while parsing a string as a [`FileId`].
#[derive(Debug, Clone, Display)]
#[display(fmt = "Specified string is not a valid FileId.")]
pub struct InvalidFileId;

impl StdError for InvalidFileId {}

impl FromStr for FileId {
    type Err = InvalidFileId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(InvalidFileId)
        } else {
            match s.parse::<Uri>() {
                Ok(url) => {
                    if let Ok(hmc) = url.clone().try_into() {
                        Ok(FileId::Hmc(hmc))
                    } else if !url.path().trim_start_matches('/').is_empty() {
                        Ok(FileId::External(url))
                    } else {
                        Ok(FileId::Id(s.to_owned()))
                    }
                }
                _ => Ok(FileId::Id(s.to_owned())),
            }
        }
    }
}

/// Wrapper type for `Vec<FileId>` so we can implement some traits.
///
/// You don't need to create this manually, since it implements `From<Vec<FileId>>`.
#[derive(new, Debug, Default, Clone, Into, From, IntoIterator)]
pub struct FileIds(Vec<FileId>);

impl From<FileIds> for Vec<String> {
    fn from(o: FileIds) -> Vec<String> {
        o.into_iter().map(|id| id.to_string()).collect()
    }
}

/// Extracts file information from a header map.
///
/// Will not fail on missing `Content-Disposition` and instead fallback to
/// `attachment; filename=unknown`.
pub fn extract_file_info_from_download_response<'a>(
    headers: &'a http::HeaderMap,
) -> Result<(&'a str, &'a HeaderValue, FileKind), &'static str> {
    let mimetype = headers
        .get(http::header::CONTENT_TYPE)
        .ok_or("server did not respond with `Content-Type` header")?;

    let mut split = headers
        .get(http::header::CONTENT_DISPOSITION)
        .map_or(Ok("attachment; filename=unknown"), |h| h.to_str())
        .map_err(|_| "server responded with non ASCII content disposition")?
        .split(';');
    let kind = match split
        .next()
        .ok_or("server did not respond with file kind")?
    {
        "attachment" => FileKind::Attachment,
        "inline" => FileKind::Inline,
        _ => return Err("server responded with invalid file kind"),
    };
    const NO_NAME: &str = "server did not respond with a file name";
    let name = split
        .next()
        .ok_or(NO_NAME)?
        .split('=')
        .nth(1)
        .ok_or(NO_NAME)?
        .trim_matches('"');

    Ok((name, mimetype, kind))
}

/// Struct that implements `serde` `Deserialize` / `Serialize` and can be used for
/// the [`/_harmony/about`](https://github.com/harmony-development/protocol/blob/main/stable/rest/rest.md#get-_harmonyabout) endpoint.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct About {
    /// the Harmony server software being hosted.
    #[serde(rename = "serverName")]
    pub server_name: String,
    /// the version of said Harmony server software.
    pub version: String,
    /// A description of why / who this server is hosted for.
    #[serde(rename = "aboutServer")]
    pub about_server: String,
    /// "message of the day", can be used to put maintenance information.
    #[serde(rename = "messageOfTheDay")]
    pub message_of_the_day: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_id() {
        const ID: &str = "654624512343";
        let file_id = FileId::from_str(ID).expect("file id parse");
        assert!(matches!(file_id, FileId::Id(_)));
        assert_eq!(ID.to_string(), file_id.to_string());
    }

    #[test]
    fn parse_hmc() {
        const HMC: &str = "hmc://chat.harmonyapp.io/654624512343";
        let file_id = FileId::from_str(HMC).expect("file id parse");
        assert!(matches!(file_id, FileId::Hmc(_)));
        assert_eq!(HMC.to_string(), file_id.to_string());
    }

    #[test]
    fn parse_uri() {
        const URI: &str = "https://media.discordapp.net/attachments/330412938277945345/801119250269339728/unknown.png";
        let file_id = FileId::from_str(URI).expect("file id parse");
        assert!(matches!(file_id, FileId::External(_)));
        assert_eq!(URI.to_string(), file_id.to_string());
    }

    #[test]
    #[should_panic(expected = "InvalidFileId")]
    fn parse_empty() {
        const EMPTY: &str = "";
        FileId::from_str(EMPTY).expect("file id parse");
    }

    fn homeserver_uri() -> Uri {
        Uri::from_static("https://chat.harmonyapp.io:2289")
    }

    #[test]
    fn join_upload_path() {
        // [tag:upload_path_create]
        assert_eq!(
            format!("{}_harmony/media/upload", homeserver_uri()),
            "https://chat.harmonyapp.io:2289/_harmony/media/upload",
        );
    }

    #[test]
    fn make_download_uri_with_id() {
        let id = "test_id_asdfasdfa";
        let uri = FileId::Id(id.to_string()).make_download_url(&homeserver_uri());
        assert_eq!(
            uri,
            format!(
                "https://chat.harmonyapp.io:2289/_harmony/media/download/{}",
                id
            ),
        );
    }

    #[test]
    fn make_download_uri_with_hmc() {
        let id = "test_hmc_asdfasdfa";
        let hmc = Hmc::new("chat.harmonyapp.io:2289", id).expect("failed to create hmc");
        let uri = FileId::Hmc(hmc).make_download_url(&homeserver_uri());
        assert_eq!(
            uri,
            format!(
                "https://chat.harmonyapp.io:2289/_harmony/media/download/{}",
                id
            ),
        );
    }

    #[test]
    fn make_download_uri_with_url() {
        let ext_url_str = "https://cdn.discordapp.com/emojis/849034023640104980.png";
        let ext_url = Uri::from_static(ext_url_str);
        let uri = FileId::External(ext_url).make_download_url(&homeserver_uri());
        assert_eq!(
            uri,
            format!(
                "https://chat.harmonyapp.io:2289/_harmony/media/download/{}",
                urlencoding::encode(ext_url_str)
            ),
        );
    }
}
