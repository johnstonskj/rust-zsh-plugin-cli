//! # zsh-plugin-cli
//!
//! A command-line tool to generate new Zsh plugin scaffolds with configurable features.
//!
//! This tool creates a complete plugin directory structure including:
//! - Main plugin source file with function tracking and unload support
//! - Optional autoloaded functions directory
//! - Optional bin directory for scripts
//! - GitHub Actions workflows for shellcheck/shellspec
//! - Git repository initialization
//!
//! ## Architecture
//!
//! The crate is organized around a simple command pattern:
//!
//! - [`cli`] - Command-line argument parsing using clap
//! - [`command`] - The [`OnceCommand`](command::OnceCommand) trait for executable commands
//! - [`error`] - Error types and conversions
//! - [`name`] - Plugin name validation
//! - [`templates`] - Template rendering using Tera
//!
//! ## Example
//!
//! ```bash
//! # Create a complete plugin with all features
//! zsh-plugin init my-plugin -t complete
//!
//! # Create a minimal plugin
//! zsh-plugin init my-plugin -t minimal
//! ```

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub(crate) mod cli;
pub(crate) mod command;
pub(crate) mod error;
pub(crate) mod name;
pub(crate) mod templates;

// ------------------------------------------------------------------------------------------------
// Imports
// ------------------------------------------------------------------------------------------------

use self::{cli::Cli, command::OnceCommand, error::Error};
use clap::Parser;
use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Command-Line Structure
// ------------------------------------------------------------------------------------------------

const COMMAND_NAME: &str = env!("CARGO_BIN_NAME");

fn main() -> Result<ExitCode, Error> {
    Cli::parse().execute()
}
