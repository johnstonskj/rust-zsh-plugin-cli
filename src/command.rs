//! Provides the [`OnceCommand`] trait for implementing executable commands.
//!
//! This module defines a simple command pattern where commands consume themselves
//! on execution, ensuring they can only be run once. This is useful for CLI
//! subcommands that perform side effects like file creation.

/// A command that consumes itself when executed.
///
/// This trait provides a clean abstraction for CLI subcommands, ensuring that
/// each command instance can only be executed once. The consuming `self` parameter
/// prevents accidental re-execution.
///
/// # Type Parameters
///
/// * `Output` - The success type returned by the command
/// * `Error` - The error type, which must implement [`std::error::Error`]
///
/// # Example
///
/// ```ignore
/// use crate::command::OnceCommand;
/// use std::process::ExitCode;
///
/// struct MyCommand {
///     name: String,
/// }
///
/// impl OnceCommand for MyCommand {
///     type Output = ExitCode;
///     type Error = std::io::Error;
///
///     fn execute(self) -> Result<Self::Output, Self::Error> {
///         println!("Hello, {}!", self.name);
///         Ok(ExitCode::SUCCESS)
///     }
/// }
/// ```
pub trait OnceCommand {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: std::error::Error;

    /// Executes the command, consuming self.
    ///
    /// # Errors
    ///
    /// Returns an error if the command fails to execute.
    fn execute(self) -> Result<Self::Output, Self::Error>;
}
