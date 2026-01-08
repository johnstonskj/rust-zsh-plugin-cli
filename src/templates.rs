use crate::{cli::InitCommand, error::Error};
use git2::Repository;
use std::{
    fs::{create_dir_all, write},
    path::{Path, PathBuf},
    process::ExitCode,
};
use tera::{Context, Tera};
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const V_GITHUB_USER: &str = "github_user";
const V_PLUGIN_DISPLAY_NAME: &str = "plugin_display_name";
const V_PLUGIN_NAME: &str = "plugin_name";
const V_PLUGIN_VAR: &str = "plugin_var";
const V_SHORT_DESCRIPTION: &str = "short_description";

const O_INCLUDE_ALIASES: &str = "include_aliases";
const O_INCLUDE_BASH_WRAPPER: &str = "include_bash_wrapper";
const O_INCLUDE_BIN_DIR: &str = "include_bin_dir";
const O_INCLUDE_FUNCTIONS_DIR: &str = "include_functions_dir";
const O_INCLUDE_GIT_INIT: &str = "include_git_init";
const O_INCLUDE_GITHUB_DIR: &str = "include_github_dir";
const O_INCLUDE_README: &str = "include_readme";
const O_INCLUDE_SHELL_CHECK: &str = "include_shell_check";
const O_INCLUDE_SHELL_SPEC: &str = "include_shell_spec";

const P_BIN_DIR: &str = "bin";
const P_DOT_GITIGNORE: &str = ".gitignore";
const P_DOT_KEEP: &str = ".gitkeep";
const P_FUNCTIONS_DIR: &str = "functions";
const P_GIHUB_DIR: &str = ".github";
const P_MAKEFILE: &str = "Makefile";
const P_README: &str = "README.md";
const P_SHELL_YML: &str = "shell.yml";
const P_WORKFLOWS_DIR: &str = "workflows";

macro_rules! report_progress {
    () => {
        print!(".");
    };
    (done) => {
        println!(" Done");
    };
}

pub(crate) fn init_new_plugin(ctx: Context, force: bool) -> Result<ExitCode, Error> {
    trace!("init_new_plugin => ctx: {ctx:?}, force: {force}");
    let mut tera = Tera::default();
    let plugin_name: &str = ctx.get(V_PLUGIN_NAME).unwrap().as_str().unwrap();

    let target_root = PathBuf::from(&format!("zsh-{plugin_name}-plugin"));
    make_directory(&target_root, force)?;

    if ctx.get(O_INCLUDE_GIT_INIT).unwrap().as_bool().unwrap() {
        make_repository(&target_root, force)?;
        render_template(
            &mut tera,
            &ctx,
            T_GIT_IGNORE,
            &target_root.join(P_DOT_GITIGNORE),
            force,
        )?;
    }

    if ctx.get(O_INCLUDE_GITHUB_DIR).unwrap().as_bool().unwrap() {
        let github = target_root.join(P_GIHUB_DIR);
        make_directory(&github, force)?;
        let workflows = github.join(P_WORKFLOWS_DIR);
        make_directory(&workflows, force)?;
        render_template(
            &mut tera,
            &ctx,
            T_GITHUB_WORFLOW_SHELL,
            &workflows.join(P_SHELL_YML),
            force,
        )?;
    }

    if ctx.get(O_INCLUDE_BIN_DIR).unwrap().as_bool().unwrap() {
        let bindir = target_root.join(P_BIN_DIR);
        make_directory(&bindir, force)?;
        render_template(
            &mut tera,
            &ctx,
            T_BIN_DIR_KEEP,
            &bindir.join(P_DOT_KEEP),
            force,
        )?;
    }

    if ctx.get(O_INCLUDE_FUNCTIONS_DIR).unwrap().as_bool().unwrap() {
        let functions = target_root.join(P_FUNCTIONS_DIR);
        make_directory(&functions, force)?;
        render_template(
            &mut tera,
            &ctx,
            T_FUNCTIONS_EXAMPLE,
            &functions.join(format!("{plugin_name}_example")),
            force,
        )?;
    }

    if ctx.get(O_INCLUDE_SHELL_CHECK).unwrap().as_bool().unwrap()
        || ctx.get(O_INCLUDE_SHELL_SPEC).unwrap().as_bool().unwrap()
    {
        render_template(
            &mut tera,
            &ctx,
            T_MAKEFILE,
            &target_root.join(P_MAKEFILE),
            force,
        )?;
    }

    if ctx.get(O_INCLUDE_BASH_WRAPPER).unwrap().as_bool().unwrap() {
        render_template(
            &mut tera,
            &ctx,
            T_PLUGIN_WRAPPER,
            &target_root.join(format!("{plugin_name}.bash")),
            force,
        )?;
    }

    if ctx.get(O_INCLUDE_README).unwrap().as_bool().unwrap() {
        render_template(
            &mut tera,
            &ctx,
            T_README,
            &target_root.join(P_README),
            force,
        )?;
    }

    render_template(
        &mut tera,
        &ctx,
        T_PLUGIN_SOURCE,
        &target_root.join(format!("{plugin_name}.plugin.zsh")),
        force,
    )?;

    report_progress!(done);

    Ok(ExitCode::SUCCESS)
}

// ------------------------------------------------------------------------------------------------
// Template Strings
// ------------------------------------------------------------------------------------------------

const T_BIN_DIR_KEEP: &str = include_str!("templates/bin/.keep");
const T_FUNCTIONS_EXAMPLE: &str = include_str!("templates/functions/name_example");
const T_GIT_IGNORE: &str = include_str!("templates/.gitignore");
const T_GITHUB_WORFLOW_SHELL: &str = include_str!("templates/.github/workflows/shell.yml");
const T_MAKEFILE: &str = include_str!("templates/Makefile");
const T_PLUGIN_SOURCE: &str = include_str!("templates/name.plugin.zsh");
const T_PLUGIN_WRAPPER: &str = include_str!("templates/name.bash");
const T_README: &str = include_str!("templates/README.md");

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_repository(path: &Path, force: bool) -> Result<(), Error> {
    trace!("make_repository => in path: {path:?}, force: {force}");

    let repo_dir = path.join(".git");
    if !repo_dir.exists() || (repo_dir.is_dir() && force) {
        if let Err(e) = Repository::init(path) {
            error!("Error initializing new Git repository, error: {e}");
            Err(e.into())
        } else {
            report_progress!();
            Ok(())
        }
    } else {
        error!("Target Git repository path {repo_dir:?} already exists");
        Err(Error::TargetExists {
            path: repo_dir.to_path_buf(),
        })
    }
}

fn make_directory(path: &Path, force: bool) -> Result<(), Error> {
    trace!("make_directory => path: {path:?}', force: {force}");

    if !path.exists() || (path.is_dir() && force) {
        create_dir_all(path)?;
        report_progress!();
        Ok(())
    } else {
        error!("Target directory {path:?} already exists");
        Err(Error::TargetExists {
            path: path.to_path_buf(),
        })
    }
}

fn render_template(
    tera: &mut Tera,
    ctx: &Context,
    template: &str,
    file_path: &Path,
    force: bool,
) -> Result<(), Error> {
    trace!("render_template => to_file: '{file_path:?}', force: {force}");

    if !file_path.exists() || (file_path.is_file() && force) {
        match tera.render_str(template, ctx) {
            Ok(content) => {
                write(file_path, content)?;
                report_progress!();
                Ok(())
            }
            Err(e) => {
                error!("failure rendering template to file {file_path:?}, error: {e}");
                Err(e.into())
            }
        }
    } else {
        error!("Target file {file_path:?} already exists");
        Err(Error::TargetExists {
            path: file_path.to_path_buf(),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Context Implementations
// ------------------------------------------------------------------------------------------------

impl From<InitCommand> for Context {
    fn from(cmd: InitCommand) -> Self {
        let mut ctx = Context::new();
        ctx.insert(O_INCLUDE_ALIASES, &!cmd.no_aliases());
        ctx.insert(O_INCLUDE_BASH_WRAPPER, &cmd.add_bash_wrapper());
        ctx.insert(O_INCLUDE_BIN_DIR, &cmd.add_bin_dir());
        ctx.insert(O_INCLUDE_FUNCTIONS_DIR, &!cmd.no_functions_dir());
        ctx.insert(O_INCLUDE_GITHUB_DIR, &!cmd.no_github_dir());
        ctx.insert(O_INCLUDE_GIT_INIT, &!cmd.no_git_init());
        ctx.insert(O_INCLUDE_README, &!cmd.no_readme());
        ctx.insert(O_INCLUDE_SHELL_CHECK, &!cmd.no_shell_check());
        ctx.insert(O_INCLUDE_SHELL_SPEC, &!cmd.no_shell_spec());
        if let Some(description) = cmd.description() {
            ctx.insert(V_SHORT_DESCRIPTION, description);
        } else {
            ctx.insert(V_SHORT_DESCRIPTION, "Add one-line description here...");
        }
        let display_name = cmd.name().to_string();
        let plugin_name = display_name.replace('-', "_");
        let plugin_var = plugin_name.to_ascii_uppercase();
        ctx.insert(V_PLUGIN_DISPLAY_NAME, &display_name);
        ctx.insert(V_PLUGIN_NAME, &plugin_name);
        ctx.insert(V_PLUGIN_VAR, &plugin_var);
        ctx.insert(V_GITHUB_USER, cmd.github_user());
        ctx.insert("_shv_start", "${");
        ctx.insert("_shv_end", "}");
        ctx
    }
}
