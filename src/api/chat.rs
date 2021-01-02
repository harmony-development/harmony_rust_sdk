use std::fmt::{self, Display, Formatter};

pub mod v1 {
    tonic::include_proto!("protocol.chat.v1");
}
pub use v1::*;

/// Describes a place in a list.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Place {
    /// Top of the list.
    Top { before: u64 },
    /// Between two items in the list.
    Between { after: u64, before: u64 },
    /// Bottom of the list.
    Bottom { after: u64 },
}

impl Place {
    /// Create a place between two other places.
    pub fn between(before: u64, after: u64) -> Self {
        Self::Between { after, before }
    }

    /// Create a place at the top of a list.
    pub fn top(before: u64) -> Self {
        Self::Top { before }
    }

    /// Create a place at the bottom of a list.
    pub fn bottom(after: u64) -> Self {
        Self::Bottom { after }
    }

    /// Get next place ID.
    pub fn next(&self) -> u64 {
        match self {
            Place::Top { before } => *before,
            Place::Between { before, after: _ } => *before,
            Place::Bottom { after: _ } => 0,
        }
    }

    /// Get previous place ID.
    pub fn previous(&self) -> u64 {
        match self {
            Place::Top { before: _ } => 0,
            Place::Between { before: _, after } => *after,
            Place::Bottom { after } => *after,
        }
    }
}

/// An invite ID.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InviteId(String);

impl InviteId {
    /// Creates an invite ID.
    ///
    /// `name` cannot be empty.
    /// If `name` is empty `None` is returned.
    pub fn new(name: impl ToString) -> Option<Self> {
        let name = name.to_string();
        if name.is_empty() {
            None
        } else {
            Some(Self(name))
        }
    }
}

impl Into<String> for InviteId {
    fn into(self) -> String {
        self.0
    }
}

impl Display for InviteId {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}
