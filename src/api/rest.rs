use std::{convert::TryInto, error::Error as StdError, str::FromStr};

use super::Hmc;
use derive_more::{Display, From, Into, IntoIterator};
use derive_new::new;
use hrpc::url::Url;
use http::HeaderValue;

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
    /// A plain ID. When you use this for a request, the `Client`s homeserver will be used.
    Id(String),
    /// An external URL. This MUST be an image according to the protocol.
    External(Url),
}

impl FileId {
    /// Get a string reference to this `FileId`.
    pub fn as_str(&self) -> &str {
        match self {
            FileId::Hmc(hmc) => hmc.as_str(),
            FileId::Id(id) => id.as_str(),
            FileId::External(url) => url.as_str(),
        }
    }
}

impl From<FileId> for String {
    fn from(o: FileId) -> String {
        match o {
            FileId::Hmc(hmc) => hmc.into(),
            FileId::Id(id) => id,
            FileId::External(url) => url.into(),
        }
    }
}

/// Error that maybe produced while parsing a string as a [`FileId`].
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
            match s.parse::<Url>() {
                Ok(url) => {
                    if let Ok(hmc) = url.clone().try_into() {
                        Ok(FileId::Hmc(hmc))
                    } else if !url.path().trim_start_matches('/').is_empty() {
                        Ok(FileId::External(url))
                    } else {
                        Err(InvalidFileId)
                    }
                }
                Err(hrpc::url::ParseError::RelativeUrlWithoutBase) => Ok(FileId::Id(s.to_owned())),
                _ => Err(InvalidFileId),
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

impl From<Hmc> for FileId {
    fn from(hmc: Hmc) -> Self {
        Self::Hmc(hmc)
    }
}

pub fn extract_file_info_from_download_response<'a>(
    headers: &'a http::HeaderMap,
) -> Result<(&'a str, &'a HeaderValue, FileKind), &'static str> {
    let mimetype = headers
        .get("Content-Type")
        .ok_or("server did not respond with `Content-Type` header")?;

    let mut split = headers
        .get("Content-Disposition")
        .ok_or("server did not respond with `Content-Disposition` header")?
        .to_str()
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
