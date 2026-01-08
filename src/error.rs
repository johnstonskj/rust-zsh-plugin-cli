/*!
Provides this crate's [`Error`] and [`Result`] types as well as helper functions.

 */

use crate::name::NameErrorKind;
use flat_error::FlatError;
use git2::Error as GitError;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::{Error as IoError, ErrorKind as IoErrorKind},
    path::PathBuf,
};
use tera::Error as TemplateError;
use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::filter::ParseError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Io { source: FlatError },
    EnvFilter { source: FlatError },
    SetGlobal { source: FlatError },
    InvalidName { kind: NameErrorKind },
    Template { source: FlatError },
    GitInit { source: FlatError },
    TargetExists { path: PathBuf },
    Multiple { sources: Vec<Error> },
    Unknown { message: String },
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Io { source } => format!("An I/O error occurred; source: {source}"),
                Self::EnvFilter { source } =>
                    format!("An error occurred parsing a tracing env-filter; source: {source}"),
                Self::SetGlobal { source } => format!(
                    "An error occurred setting the global tracing subscriber; source:{source}"
                ),
                Self::InvalidName { kind } =>
                    format!("An error occured parsing a Name value; kind: {kind:?}"),
                Self::Template { source } =>
                    format!("An error occurred parsing or rendering a template; source:{source}"),
                Self::GitInit { source } => format!(
                    "An error occurred initializing the new Git repository; source: {source}"
                ),
                Self::TargetExists { path } => format!(
                    "An error occurred generating a template: target path {path:?} already exists"
                ),
                Self::Multiple { sources } => {
                    format!(
                        "Multiple errors occurred:\n{}",
                        sources
                            .iter()
                            .enumerate()
                            .map(|(i, e)| format!("{i:<3}. {e}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
                Self::Unknown { message } =>
                    format!("An unknown error occurred; message: {message}"),
            }
        )
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io { source } => Some(source),
            Self::EnvFilter { source } => Some(source),
            Self::SetGlobal { source } => Some(source),
            Self::GitInit { source } => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations From
// ------------------------------------------------------------------------------------------------

impl From<IoError> for Error {
    fn from(source: IoError) -> Self {
        Self::Io {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<IoErrorKind> for Error {
    fn from(source: IoErrorKind) -> Self {
        let error: IoError = source.into();
        Self::Io {
            source: FlatError::from_any(&error),
        }
    }
}

impl From<SetGlobalDefaultError> for Error {
    fn from(source: SetGlobalDefaultError) -> Self {
        Self::SetGlobal {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<ParseError> for Error {
    fn from(source: ParseError) -> Self {
        Self::EnvFilter {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<NameErrorKind> for Error {
    fn from(kind: NameErrorKind) -> Self {
        Self::InvalidName { kind }
    }
}

impl From<TemplateError> for Error {
    fn from(source: TemplateError) -> Self {
        Self::Template {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<GitError> for Error {
    fn from(source: GitError) -> Self {
        Self::GitInit {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<Vec<Error>> for Error {
    fn from(sources: Vec<Error>) -> Self {
        Self::Multiple { sources }
    }
}

impl From<&[Error]> for Error {
    fn from(sources: &[Error]) -> Self {
        Self::Multiple {
            sources: sources.to_vec(),
        }
    }
}

impl FromIterator<Error> for Error {
    fn from_iter<I: IntoIterator<Item = Error>>(iter: I) -> Self {
        Self::Multiple {
            sources: iter.into_iter().collect(),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Unknown { message }
    }
}
