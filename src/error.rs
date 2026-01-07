/*!
Provides this crate's [`Error`] and [`Result`] types as well as helper functions.

 */

use flat_error::FlatError;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::{Error as IoError, ErrorKind as IoErrorKind}, path::PathBuf,
};
use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::filter::ParseError;
use crate::name::NameErrorKind;
use tera::Error as TemplateError;
use git2::Error as GitError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    IoError {
        source: FlatError,
    },
    EnvFilterError {
        source: FlatError,
    },
    SetGlobalError {
        source: FlatError,
    },
    InvalidNameError {
        kind: NameErrorKind,
    },
    TemplateError {
        source: FlatError,
    },
    GitInitError {
        source: FlatError,
    },
    TargetExistsError {
        path: PathBuf,
    },
    MultipleErrors {
        sources: Vec<Error>,
    },
    Unknown {
        message: String,
    },
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
                Self::IoError { source } => format!("An I/O error occurred; source: {source}"),
                Self::EnvFilterError { source } =>
                    format!("An error occurred parsing a tracing env-filter; source: {source}"),
                Self::SetGlobalError { source } => format!(
                    "An error occurred setting the global tracing subscriber; source:{source}"
                ),
                Self::InvalidNameError { kind } => format!(
                    "An error occured parsing a Name value; kind: {kind:?}"
                ),
                Self::TemplateError { source } => format!(
                    "An error occurred parsing or rendering a template; source:{source}"
                ),
                Self::GitInitError { source } => format!(
                    "An error occurred initializing the new Git repository; source: {source}"
                ),
                Self::TargetExistsError { path } => format!(
                    "An error occurred generating a template: target path {path:?} already exists"
                ),
                Self::MultipleErrors { sources } => {
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
            Self::IoError { source } => Some(source),
            Self::EnvFilterError { source } => Some(source),
            Self::SetGlobalError { source } => Some(source),
            Self::GitInitError { source } => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations From
// ------------------------------------------------------------------------------------------------

impl From<IoError> for Error {
    fn from(source: IoError) -> Self {
        Self::IoError {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<IoErrorKind> for Error {
    fn from(source: IoErrorKind) -> Self {
        let error: IoError = source.into();
        Self::IoError {
            source: FlatError::from_any(&error),
        }
    }
}

impl From<SetGlobalDefaultError> for Error {
    fn from(source: SetGlobalDefaultError) -> Self {
        Self::SetGlobalError {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<ParseError> for Error {
    fn from(source: ParseError) -> Self {
        Self::EnvFilterError {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<NameErrorKind> for Error {
    fn from(kind: NameErrorKind) -> Self {
        Self::InvalidNameError { kind }
    }
}

impl From<TemplateError> for Error {
    fn from(source: TemplateError) -> Self {
        Self::TemplateError {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<GitError> for Error {
    fn from(source: GitError) -> Self {
        Self::GitInitError {
            source: FlatError::from_any(&source),
        }
    }
}

impl From<Vec<Error>> for Error {
    fn from(sources: Vec<Error>) -> Self {
        Self::MultipleErrors { sources }
    }
}

impl From<&[Error]> for Error {
    fn from(sources: &[Error]) -> Self {
        Self::MultipleErrors {
            sources: sources.to_vec(),
        }
    }
}

impl FromIterator<Error> for Error {
    fn from_iter<I: IntoIterator<Item = Error>>(iter: I) -> Self {
        Self::MultipleErrors {
            sources: iter.into_iter().collect(),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::Unknown { message }
    }
}
