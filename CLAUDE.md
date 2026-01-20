# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Implementation 

This tool is implemented in Rust with the following major dependencies for CLI development.

- `clap`
- `clap-verbosity-flag`
- `clap_complete`
- `tracing`
- `tracing-subscriber`

### Build Commands

```bash
cargo build              # Build the project
cargo test               # Run all tests
cargo test name::tests   # Run tests for a specific module
cargo run -- --help      # Run the CLI with arguments
cargo clippy             # Run linter
cargo fmt                # Format code
```

## Architecture

This is a CLI tool (`zsh-plugin`) that scaffolds Zsh plugin directory structures using templates.

### Module Organization

- **main.rs** - Entry point; parses CLI args and executes the command
- **cli.rs** - Clap-based argument parsing; defines `Cli`, `Commands` enum, `InitCommand` struct, and `Template` enum (Minimal/Simple/Complete)
- **command.rs** - `OnceCommand` trait for commands that consume themselves on execution
- **error.rs** - Custom `Error` enum with variants for different failure modes; uses `FlatError` for wrapping third-party errors
- **name.rs** - `Name` newtype for validated plugin names (must start with letter, contain only alphanumeric/hyphen/underscore)
- **templates.rs** - Template rendering using Tera; `init_new_plugin()` orchestrates file/directory creation

### Key Patterns

**OnceCommand trait**: Commands implement `execute(self) -> Result<Output, Error>`, consuming themselves to prevent re-execution. The execution flow is: `Cli::execute()` → `Commands::execute()` → `InitCommand::execute()`.

**Template Context**: `InitCommand` converts to `tera::Context` via `From` impl. Context keys are defined as constants (`V_*` for values, `O_*` for options).

**Template files**: Located in `src/templates/`, embedded via `include_str!()`. Templates use Tera syntax with variables like `{{ plugin_name }}`.

## Style Guidelines

### Code Style

1. Before commit: 
   1. ensure all tests pass,
   2. ensure all code is formatted correctly using `cargo fmt`,
   3. ensure all lint issues are addressed from `cargo clippy`.

### Commit Style

Commit messages use *conventional commits* format and **must** conform to the [specification](https://www.conventionalcommits.org/en/v1.0.0/#specification).

```text
<type>[optional scope][optional indicator]: <description>

[optional body]

[optional footer(s)]
```

