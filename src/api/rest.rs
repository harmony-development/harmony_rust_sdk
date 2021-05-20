use std::{convert::TryInto, error::Error as StdError, str::FromStr};

use super::Hmc;
use derive_more::{Display, From, Into, IntoIterator};
use derive_new::new;
use hrpc::url::Url;

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
