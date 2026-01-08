use crate::error::Error;
use std::{fmt::Display, str::FromStr};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Name(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum NameErrorKind {
    Empty,
    InvalidInitialChar,
    InvalidChar,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Name
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<Name> for String {
    fn from(id: Name) -> Self {
        id.0
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            error!("Name::from_str; value is empty");
            Err(NameErrorKind::Empty.into())
        } else if !s.chars().next().unwrap().is_ascii_alphabetic() {
            error!("Name::from_str; initial character must be alphabetic");
            Err(NameErrorKind::InvalidInitialChar.into())
        } else if !(s[1..])
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            error!("Name::from_str; character must be alphanumeric, '-', or '_'");
            Err(NameErrorKind::InvalidChar.into())
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ NameErrorKind
// ------------------------------------------------------------------------------------------------

impl Display for NameErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameErrorKind::Empty => write!(f, "Name cannot be empty"),
            NameErrorKind::InvalidInitialChar => {
                write!(f, "Initial character must be an ASCII alphabetic character")
            }
            NameErrorKind::InvalidChar => {
                write!(f, "Characters must be ASCII alphanumeric, '-', or '_'")
            }
        }
    }
}
