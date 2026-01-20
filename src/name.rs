//! Plugin name validation and representation.
//!
//! This module provides the [`Name`] type for representing validated plugin names.
//! Plugin names must follow specific rules to ensure they work correctly as
//! Zsh identifiers and file names.
//!
//! # Naming Rules
//!
//! A valid plugin name must:
//! - Start with an ASCII alphabetic character (`a-z` or `A-Z`)
//! - Contain only ASCII alphanumeric characters, hyphens (`-`), or underscores (`_`)
//!
//! # Examples
//!
//! ```ignore
//! use std::str::FromStr;
//!
//! // Valid names
//! let name: Name = "my-plugin".parse().unwrap();
//! let name: Name = "MyPlugin_v2".parse().unwrap();
//!
//! // Invalid names
//! assert!("2plugin".parse::<Name>().is_err());   // starts with number
//! assert!("-plugin".parse::<Name>().is_err());   // starts with hyphen
//! assert!("my plugin".parse::<Name>().is_err()); // contains space
//! ```

use crate::error::Error;
use std::{fmt::Display, str::FromStr};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A validated plugin name.
///
/// `Name` is a newtype wrapper around `String` that ensures the contained
/// value is a valid plugin name according to the naming rules.
///
/// # Examples
///
/// ```ignore
/// // Parse from a string
/// let name: Name = "my-plugin".parse()?;
///
/// // Use as a string reference
/// let s: &str = name.as_ref();
///
/// // Convert to owned String
/// let owned: String = name.into();
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Name(String);

/// Describes the kind of error that occurred when parsing a [`Name`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum NameErrorKind {
    /// The name string was empty.
    Empty,
    /// The first character was not an ASCII alphabetic character.
    InvalidInitialChar,
    /// A character other than alphanumeric, hyphen, or underscore was found.
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
        let mut chars = s.chars();
        match chars.next() {
            None => {
                error!("Name::from_str; value is empty");
                Err(NameErrorKind::Empty.into())
            }
            Some(first) if !first.is_ascii_alphabetic() => {
                error!("Name::from_str; initial character must be alphabetic");
                Err(NameErrorKind::InvalidInitialChar.into())
            }
            Some(_) if !chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') => {
                error!("Name::from_str; character must be alphanumeric, '-', or '_'");
                Err(NameErrorKind::InvalidChar.into())
            }
            Some(_) => Ok(Self(s.to_string())),
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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_simple_name() {
        let name: Result<Name, _> = "myplugin".parse();
        assert!(name.is_ok());
        assert_eq!(name.unwrap().to_string(), "myplugin");
    }

    #[test]
    fn valid_name_with_hyphens() {
        let name: Result<Name, _> = "my-plugin".parse();
        assert!(name.is_ok());
        assert_eq!(name.unwrap().to_string(), "my-plugin");
    }

    #[test]
    fn valid_name_with_underscores() {
        let name: Result<Name, _> = "my_plugin".parse();
        assert!(name.is_ok());
        assert_eq!(name.unwrap().to_string(), "my_plugin");
    }

    #[test]
    fn valid_name_with_numbers() {
        let name: Result<Name, _> = "plugin2".parse();
        assert!(name.is_ok());
        assert_eq!(name.unwrap().to_string(), "plugin2");
    }

    #[test]
    fn valid_name_mixed() {
        let name: Result<Name, _> = "My-Plugin_v2".parse();
        assert!(name.is_ok());
        assert_eq!(name.unwrap().to_string(), "My-Plugin_v2");
    }

    #[test]
    fn valid_single_char() {
        let name: Result<Name, _> = "a".parse();
        assert!(name.is_ok());
    }

    #[test]
    fn invalid_empty_name() {
        let name: Result<Name, _> = "".parse();
        assert!(name.is_err());
        if let Err(Error::InvalidName { kind }) = name {
            assert_eq!(kind, NameErrorKind::Empty);
        } else {
            panic!("Expected InvalidName error with Empty kind");
        }
    }

    #[test]
    fn invalid_starts_with_number() {
        let name: Result<Name, _> = "2plugin".parse();
        assert!(name.is_err());
        if let Err(Error::InvalidName { kind }) = name {
            assert_eq!(kind, NameErrorKind::InvalidInitialChar);
        } else {
            panic!("Expected InvalidName error with InvalidInitialChar kind");
        }
    }

    #[test]
    fn invalid_starts_with_hyphen() {
        let name: Result<Name, _> = "-plugin".parse();
        assert!(name.is_err());
        if let Err(Error::InvalidName { kind }) = name {
            assert_eq!(kind, NameErrorKind::InvalidInitialChar);
        } else {
            panic!("Expected InvalidName error with InvalidInitialChar kind");
        }
    }

    #[test]
    fn invalid_starts_with_underscore() {
        let name: Result<Name, _> = "_plugin".parse();
        assert!(name.is_err());
        if let Err(Error::InvalidName { kind }) = name {
            assert_eq!(kind, NameErrorKind::InvalidInitialChar);
        } else {
            panic!("Expected InvalidName error with InvalidInitialChar kind");
        }
    }

    #[test]
    fn invalid_contains_space() {
        let name: Result<Name, _> = "my plugin".parse();
        assert!(name.is_err());
        if let Err(Error::InvalidName { kind }) = name {
            assert_eq!(kind, NameErrorKind::InvalidChar);
        } else {
            panic!("Expected InvalidName error with InvalidChar kind");
        }
    }

    #[test]
    fn invalid_contains_special_char() {
        let name: Result<Name, _> = "my@plugin".parse();
        assert!(name.is_err());
        if let Err(Error::InvalidName { kind }) = name {
            assert_eq!(kind, NameErrorKind::InvalidChar);
        } else {
            panic!("Expected InvalidName error with InvalidChar kind");
        }
    }

    #[test]
    fn as_ref_returns_str() {
        let name: Name = "myplugin".parse().unwrap();
        let s: &str = name.as_ref();
        assert_eq!(s, "myplugin");
    }

    #[test]
    fn into_string() {
        let name: Name = "myplugin".parse().unwrap();
        let s: String = name.into();
        assert_eq!(s, "myplugin");
    }
}
