# Package/Tool `zsh-plugin`

A command-line tool to generate new Zsh plugins.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![Rust Workflow](https://github.com/johnstonskj/rust-zsh-plugin-cli/actions/workflows/rust.yml/badge.svg)](<https://github.com/johnstonskj/rust-zsh-plugin-cli/actions/workflows/rust.yml>)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-zsh-plugin-cli/actions/workflows/security-audit.yml/badge.svg)](<https://github.com/johnstonskj/rust-zsh-plugin-cli/actions/workflows/security-audit.yml>)
[![crates.io](https://img.shields.io/crates/v/zsh-plugin.svg)](https://crates.io/crates/zsh-plugin)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-zsh-plugin-cli.svg)](<https://github.com/johnstonskj/rust-zsh-plugin-cli/stargazers>)

This tool scaffolds Zsh plugins with best practices built-in, including function
tracking for clean unloading, optional alias support, autoloaded functions, and
CI/CD workflows for shellcheck and shellspec. Choose from _minimal_, _simple_, or
_complete_ templates to match your plugin's complexity.

## Install

```bash
❱ cargo install zsh-plugin
```

```bash
❱ zsh-plugin --help
A command-line tool to generate new Zsh plugins.

Usage: zsh-plugin [OPTIONS] <COMMAND>

Commands:
  init  Initialize a new Zsh plugin structure
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

## Command `init`

```bash
❯ zsh-plugin init --help                                                                              
Initialize a new Zsh plugin structure

The resulting plugin contains the following content.

...

Usage: zsh-plugin init [OPTIONS] --github-user <GITHUB_USER> <NAME>

Arguments:
  <NAME>
          The name of the new plugin.
          
          Plugin names are restricted to a "safe" subset corresponding to the following regular expression `\[a-zA-Z\]\[a-zA-Z0-9_-\]``.

Options:
  -f, --force
          Force over-writing of existing files.
          
          If not set, the tool will fail when target directories or files exist.

  -v, --verbose...
          Increase logging verbosity.

  -q, --quiet...
          Decrease logging verbosity.

  -t, --template <TEMPLATE>
          A pre-configured template to use.

          Possible values:
          - minimal:  Minimal plugin structure
          - simple:   Simple in-line function plugin structure
          - complete: Complete plugin structure with all optional components included

  -a, --add-bin-dir
          Add a 'bin' sub-directory for plugin-specific binaries/scripts.

  -w, --add-bash-wrapper
          Add a Bash wrapper file to call the plugin from Bash scripts.

  -A, --no-aliases
          Do not include support for tracking aliases defined by the plugin.

  -C, --no-shell-check
          Do not include support for linting using shellcheck.
          
          Add linting steps to the Makefile and shell.yml (Github Action) files.

  -D, --no-shell-doc
          Do not include support for documentation generation with shdoc.
          
          Add documentation steps to the Makefile.

  -F, --no-functions-dir
          Do not include a 'functions' sub-directory and example file.

  -G, --no-git-init
          Do not initialize Git in the generated plugin.
          
          By default the created plugin directory is also initialized as a new Git repository. This option also stops creation a generic .gitignore file.

  -H, --no-github-dir
          Do not include a '.github' sub-directory.
          
          By default the created plugin includes a .github/worflows directory with a file shell.yml that defines a Github Actions workflow. Note that if both no-shell-check and no-shell_test options are set the workflow file is not created as it would effectively be a no-op.

  -R, --no-readme
          Do not include a README file.

  -S, --no-shell-spec
          Do not include support for testing using shellspec.
          
          Add testing steps to the Makefile and shell.yml (Github Action) files.

  -u, --github-user <GITHUB_USER>
          Set the name of the Github user for inclusion in README.md
          
          [env: USER=simon]

  -Z, --use-plain-plugins
          Do not use the `zplugins` plugin manager for support functions.

  -d, --description <DESCRIPTION>
          Short description of the plugin.
          
          This description is added to the plugin source and README.md files.

  -h, --help
          Print help (see a summary with '-h')
```

### Results

Initialize a new Zsh plugin structure in the directory `zsh-NAME-plugin`.
he resulting plugin contains the following content.

1. A file `NAME.plugin.zsh` which consists of the main plugin
   lifecycle functions.
   1. A function `NAME_plugin_init` is included with comments to show
      how to save environment variables, add aliases, and add to either
      `path` or `fpath`.
   2. A function `NAME_plugin_unload` is included with comments to
      demonstrate custom clean-up actions.
   3. An example global variable `NAME_EXAMPLE` set during _source_ time.
   4. A call during _source_ time to set any dependencies the plugin
      wishes to declare.

2. If the option `add-bash-wrapper` is defined, a file `NAME.bash`
   is included which provides an entry point for Bash users to load the
   plugin.

3. A directory `.github/workflows` and a Github Actions script named
   `shell.yml` to automate shellcheck and shellspec. Generation will be
   skipped if the `no-github-dir` option is checked **or** both the options
   `no-shell-check` and `no-shell-spec` are set as the workflow then has
   nothing to do.

4. A directory `functions` with an example autoloaded function
   named `NAME_example`. Generation will be skipped if the
   `no-functions-dir` option is set.

5. If the option `add-bin-dir` is set an empty `bin` directory for
   plugin specific binaries is created.

6. A file `.gitignore`. Generation will be skipped if both the options
   `no-shell-check` and `no-shell-spec`are set.

7. A directory `doc`. Generation will be skipped if the `no-shell-doc`
   option is set.

8. A file `Makefile` for GNU Make. Generation will be skipped if
   the options `no-shell-check`, `no-shell-doc`, and `no-shell-spec`
   are all true.

9. A file `README.md` containing only a basic skeleton. Generation will be
   skipped if the `no-readme` is set.

### Templates

Rather than setting all options manually, three templates are provided with
pre-defined selection of settings. These are described in the table below.

| Feature / Template  | minimal | simple | complete |
| ------------------- | ------- | ------ | -------- |
| `add-bin-dir`       | false   | false  | true     |
| `add-bash-wrapper`  | false   | false  | true     |
| `no-aliases`        | true    | false  | false    |
| `no-functions-dir`  | true    | true   | false    |
| `no-git-init`       | false   | false  | false    |
| `no-github-dir`     | true    | true   | false    |
| `no-readme`         | true    | false  | false    |
| `no-shell-check`    | true    | false  | false    |
| `no-shell-doc`      | true    | false  | false    |
| `no-shell-spec`     | true    | false  | false    |

### Example

Given the following execution:

```bash
❱ zsh-plugin init containers --add-bash-wrapper --add-bin-dir
........ Done
```

A complete plugin, with all possible content, is created as follows:

```text
─ zsh-containers-plugin
  ├─ .github/                # unless no-github-dir OR
  │  └─ workflows/           #   no-shell-check AND no-shell-check AND no-shell-spec
  │     └─ shell.yml
  ├─ bin/                    # when   add-bin-dir
  │  └─ .gitkeep
  ├─ functions/              # unless no-functions-dir
  │  └─ containers_example
  ├─ .git                    # unless no-git-init
  ├─ .gitignore              # unless no-git-init
  ├─ Makefile                # unless no-shell-check AND no-shell-check AND no-shell-spec
  ├─ mkdoc.zsh               # unless no-shell-doc
  ├─ README.md
  ├─ containers.bash         # when   add-bash-wrapper
  └─ containers.plugin.zsh  
```

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/LICENSE-MIT).

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

For information on contributing to this project, see the following.

1. Project [Code of Conduct](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/CODE_OF_CONDUCT.md).
1. Project [Contribution Guidelines](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/CONTRIBUTING.md).
1. Project [TODO Items](<https://github.com/johnstonskj/rust-zsh-plugin/issues>) in Issues.
1. Repository [Change Log](https://github.com/johnstonskj/rust-zsh-plugin/blob/main/CHANGELOG.md).
