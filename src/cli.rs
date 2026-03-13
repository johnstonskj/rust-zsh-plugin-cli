use crate::{command::OnceCommand, error::Error, name::Name, templates::init_new_plugin};
use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::filter::EnvFilter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(version, name = super::COMMAND_NAME, about, long_about = None)]
pub(crate) struct Cli {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum Commands {
    /// Initialize a new Zsh plugin structure
    ///
    /// The resulting plugin contains the following content.
    ///
    /// 1. A file `NAME.plugin.zsh` which consists of the main plugin
    ///    lifecycle functions.
    ///
    /// 2. A function `NAME_plugin_init` is included with comments to show
    ///    how to save environment variables, add aliases, and add to either
    ///    `path` or `fpath`.
    ///
    /// 3. A function `NAME_plugin_unload` is included with comments to
    ///    demonstrate custom clean-up actions.
    ///
    /// 4. An example global variable `NAME_EXAMPLE` set during _source_ time.
    ///
    /// 5. A call during _source_ time to set any dependencies the plugin
    ///    wishes to declare.
    ///
    /// 6. If the option `add-bash-wrapper` is defined, a file `NAME.bash`
    ///    is included which provides an entry point for Bash users to load the
    ///    plugin.
    ///
    /// 7. A directory `.github/workflows` and a Github Actions script named
    ///    `shell.yml` to automate shellcheck and shellspec. Generation will be
    ///    skipped if the `no-github-dir` option is checked **or** both the options
    ///    `no-shell-check` and `no-shell-spec` are set as the workflow then has
    ///    nothing to do.
    ///
    /// 8. A directory `functions` with an example autoloaded function
    ///    named `NAME_example`. Generation will be skipped if the
    ///    `no-functions-dir` option is set.
    ///
    /// 9. If the option `add-bin-dir` is set an empty `bin` directory for
    ///    plugin specific binaries is created.
    ///
    /// 10. A file `.gitignore`. Generation will be skipped if both the options
    ///     `no-shell-check` and `no-shell-spec`are set.
    ///
    /// 11. A directory `doc`. Generation will be skipped if the `no-shell-doc`
    ///     option is set.
    ///
    /// 12. A file `Makefile` for GNU Make. Generation will be skipped if
    ///     the options `no-shell-check`, `no-shell-doc`, and `no-shell-spec`
    ///     are all true.
    ///
    /// 13. A file `README.md` containing only a basic skeleton. Generation will be
    ///     skipped if the `no-readme` is set.
    Init(InitCommand),
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitCommand {
    /// Force over-writing of existing files.
    ///
    /// If not set, the tool will fail when target directories or files exist.
    #[arg(long, short = 'f', action)]
    force: bool,

    #[arg(long, short = 't')]
    template: Option<Template>,

    /// Add a 'bin' sub-directory for plugin-specific binaries/scripts.
    ///
    /// The zplugins framework automatically adds this directory to
    /// 'PATH' if present.
    #[arg(long, short = 'a', action, conflicts_with = "template")]
    add_bin_dir: bool,

    /// Add a Bash wrapper file to call the plugin from Bash scripts.
    ///
    #[arg(long, short = 'w', action, conflicts_with = "template")]
    add_bash_wrapper: bool,

    /// Do not include generation of alias examples within the plugin.
    ///
    /// Examples are usually added to the plugin's main file, these can
    /// be skipped if you have no intention of using aliases.
    #[arg(long, short = 'A', action, conflicts_with = "template")]
    no_aliases: bool,

    /// Do not include support for linting using the shellcheck tool.
    ///
    /// Add linting rules to the 'Makefile' and 'shell.yml' (Github Action) files.
    #[arg(long, short = 'C', action, conflicts_with = "template")]
    no_shell_check: bool,

    /// Do not include support for documentation generation with the
    /// shdoc tool.
    ///
    /// Add documentation steps to the 'Makefile'. The plugin file and
    /// all functions will include basic shelldoc documentation whether
    /// this flag is set or not.
    #[arg(long, short = 'D', action, conflicts_with = "template")]
    no_shell_doc: bool,

    /// Do not include a 'functions' sub-directory and example file.
    ///
    /// By default a directory `functions` is created and a file named
    /// 'NAME_example' is included with an example function autoloaded
    /// by zplugins.
    /// The zplugins framework automatically adds this directory to
    /// 'FPATH' if present.
    #[arg(long, short = 'F', action, conflicts_with = "template")]
    no_functions_dir: bool,

    /// Do not initialize Git in the generated plugin.
    ///
    /// By default the created plugin directory is also initialized as a new
    /// Git repository. This option also stops creation of any '.gitignore'
    /// file.
    #[arg(long, short = 'G', action, conflicts_with = "template")]
    no_git_init: bool,

    /// Do not include a '.github' sub-directory.
    ///
    /// By default the created plugin includes a '.github/worflows' directory
    /// with a file 'shell.yml' that defines a Github Actions workflow. Note
    /// that if both 'no-shell-check' and 'no-shell_test' options are set the
    /// workflow file is not created as it would effectively be a no-op.
    #[arg(long, short = 'H', action, conflicts_with = "template")]
    no_github_dir: bool,

    /// Do not include a README.md file.
    #[arg(long, short = 'R', action, conflicts_with = "template")]
    no_readme: bool,

    /// Do not include support for testing using the shellspec tool.
    ///
    /// Add testing steps to the 'Makefile' and 'shell.yml' (Github Action) files.
    #[arg(long, short = 'S', action, conflicts_with = "template")]
    no_shell_spec: bool,

    /// Set the name of the Github user for inclusion in 'README.md'.
    #[arg(long, short = 'u', env = "USER")]
    github_user: String,

    /// Do not use the `zplugins` plugin manager for support functions.
    ///
    /// The zplugins manager provides a lot of features such as autoloading
    /// functions, adding certain directories to 'path' and 'fpath' if present
    /// and tracking global variables and aliases so they are automatically
    /// cleaned up when a plugin is unloaded. Without this framework the
    /// generated plugin has a lot of code to emulate some of these features.
    #[arg(long, short = 'Z')]
    use_plain_plugins: bool,

    /// Short description of the plugin.
    ///
    /// This description is added to the plugin source and 'README.md' files.
    #[arg(long, short = 'd')]
    description: Option<String>,

    /// The name of the new plugin.
    ///
    /// Plugin names are restricted to a "safe" subset corresponding to the
    /// following regular expression `\[a-zA-Z\]\[a-zA-Z0-9_-\]``.
    name: Name,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ValueEnum)]
pub(crate) enum Template {
    /// Minimal plugin structure.
    ///
    /// The generated plugin contains no binary or functions directories,
    /// no Git directory, no GitHub workflows, and no support for aliases,
    /// shellcheck or  shellspec.
    Minimal,

    /// Simple in-line function plugin structure.
    ///
    /// The generated plugin contains support for aliases, shellcheck and
    /// shellspec, but all functions are defined in-line in the main plugin.
    /// It does not include the binary or functions directories, or
    /// GitHub workflows.
    Simple,

    /// Complete plugin structure with all optional components included.
    Complete,
}

// ------------------------------------------------------------------------------------------------
// Command Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for Cli {
    type Output = ExitCode;
    type Error = Error;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        init_tracing(self.verbosity)?;
        self.cmd.clone().execute()
    }
}

impl OnceCommand for Commands {
    type Output = ExitCode;
    type Error = Error;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        match self {
            Commands::Init(init_command) => init_command.execute(),
        }
    }
}

impl OnceCommand for InitCommand {
    type Output = ExitCode;
    type Error = Error;

    fn execute(mut self) -> Result<Self::Output, Self::Error> {
        let force = self.force();
        self.normalize();
        match init_new_plugin(self.into(), force) {
            Ok(code) => Ok(code),
            Err(Error::GitInit { source }) => {
                eprintln!(
                    r#"Initialization failed due to Git repository initialization error.
├─ Error: {source}
└─ Help: Ensure that Git is installed and accessible, or use the '--no-git-init' option to skip Git initialization."#
                );
                Ok(ExitCode::FAILURE)
            }
            Err(Error::InvalidName { kind }) => {
                eprintln!(
                    r#"Initialization failed due to invalid plugin name.
├─ Error: {kind}
└─ Help: Plugin names must start with a letter and can only contain letters, digits, hyphens and underscores."#
                );
                Ok(ExitCode::FAILURE)
            }
            Err(Error::TargetExists { path }) => {
                eprintln!(
                    r#"Initialization failed as the target file or directory already exists.
├─ Path: {path:?}
└─ Help: Use the '--force' option to overwrite existing files and directories."#
                );
                Ok(ExitCode::FAILURE)
            }
            Err(Error::Template { source }) => {
                eprintln!(
                    r#"Initialization failed due to a template rendering error.
├─ Error: {source}
└─ Help: Ensure that the template files are correct and try again."#
                );
                Ok(ExitCode::FAILURE)
            }
            Err(e) => {
                eprintln!(
                    r#"An error initializing the new plugin
└─ Error: {e}"#
                );
                Ok(ExitCode::FAILURE)
            }
        }
    }
}

impl InitCommand {
    pub(crate) fn force(&self) -> bool {
        self.force
    }
    pub(crate) fn add_bash_wrapper(&self) -> bool {
        self.add_bash_wrapper
    }
    pub(crate) fn no_functions_dir(&self) -> bool {
        self.no_functions_dir
    }
    pub(crate) fn add_bin_dir(&self) -> bool {
        self.add_bin_dir
    }
    pub(crate) fn no_git_init(&self) -> bool {
        self.no_git_init
    }
    pub(crate) fn no_github_dir(&self) -> bool {
        self.no_github_dir
    }
    pub(crate) fn no_aliases(&self) -> bool {
        self.no_aliases
    }
    pub(crate) fn no_readme(&self) -> bool {
        self.no_readme
    }
    pub(crate) fn no_shell_check(&self) -> bool {
        self.no_shell_check
    }
    pub(crate) fn no_shell_doc(&self) -> bool {
        self.no_shell_doc
    }
    pub(crate) fn no_shell_spec(&self) -> bool {
        self.no_shell_spec
    }
    pub(crate) fn use_plain_plugins(&self) -> bool {
        self.use_plain_plugins
    }
    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub(crate) fn github_user(&self) -> &str {
        &self.github_user
    }
    pub(crate) fn name(&self) -> &Name {
        &self.name
    }
    fn normalize(&mut self) {
        match self.template {
            Some(Template::Minimal) => {
                self.add_bin_dir = false;
                self.add_bash_wrapper = false;
                self.no_aliases = true;
                self.no_functions_dir = true;
                self.no_github_dir = true;
                self.no_git_init = false;
                self.no_readme = true;
                self.no_shell_check = true;
                self.no_shell_doc = true;
                self.no_shell_spec = true;
            }
            Some(Template::Simple) => {
                self.add_bin_dir = false;
                self.add_bash_wrapper = false;
                self.no_aliases = false;
                self.no_functions_dir = true;
                self.no_github_dir = true;
                self.no_git_init = false;
                self.no_readme = false;
                self.no_shell_check = false;
                self.no_shell_doc = false;
                self.no_shell_spec = false;
            }
            Some(Template::Complete) | None => {
                self.add_bin_dir = true;
                self.add_bash_wrapper = true;
                self.no_aliases = false;
                self.no_functions_dir = false;
                self.no_github_dir = false;
                self.no_git_init = false;
                self.no_readme = false;
                self.no_shell_check = false;
                self.no_shell_doc = false;
                self.no_shell_spec = false;
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(log_level: clap_verbosity_flag::Verbosity) -> Result<(), Error> {
    let log_level: LevelFilter = log_level.into();
    let filter = EnvFilter::from_default_env().add_directive(
        format!("{}={}", module_path!(), log_level)
            .parse()
            .map_err(|e| {
                error!("Error parsing trace env-filter expression; source: {e}");
                Error::from(e)
            })?,
    );

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_max_level(log_level)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .pretty()
        .init();

    Ok(())
}
