# Package/Tool `zsh-plugin`

A command-line tool to generate new Zsh plugins.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![Rust Workflow](https://github.com/johnstonskj/rust-zsh-plugin/actions/workflows/rust.yml/badge.svg)](<https://github.com/johnstonskj/rust-zsh-plugin/actions/workflows/rust.yml>)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-zsh-plugin/actions/workflows/security-audit.yml/badge.svg)](<https://github.com/johnstonskj/rust-zsh-plugin/actions/workflows/security-audit.yml>)
[![Coverage Status](https://codecov.io/github/johnstonskj/rust-zsh-plugin/graph/badge.svg?token=1HGN6M4KIT)](<https://codecov.io/github/johnstonskj/rust-zsh-plugin>)
[![crates.io](https://img.shields.io/crates/v/zsh-plugin.svg)](https://crates.io/crates/zsh-plugin)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/zsh-plugin)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-zsh-plugin.svg)](<https://github.com/johnstonskj/rust-zsh-plugin/stargazers>)

Add a longer description here.

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

Initialize a new Zsh plugin structure in the directory `zsh-NAME-plugin`.
The resulting plugin contains the following content.

1. A file `NAME.plugin.zsh` which consists of the main plugin logic
   including support for autoloaded functions in the `functions`
   directory, or (if `no-functions-dir` is set) functions defined inline.
   Function `_NAME_remember_fn` keeps track of all plugin-defined 
   functions so they can be unset during the function 
   `NAME_plugin_unload`. Similarly, a function `_NAME_define_alias`
   is included, unless `no-aliases` is set, that both defines the alias
   and keeps track of all plugin-defined aliases so they can be unset
   during the function `NAME_plugin_unload`. A function `NAME_plugin_init`
   is included, unless both `no-functions-dir` and `no-bin-dir` are set,
   which sets up the corresponding `FPATH` and `PATH` variables. Finally,
   the function `NAME_plugin_unload` is defined and contains the logic
   to unfunction all the remembered functions, unalias all the remembered
   aliases, remove entries from `FPATH` and `PATH` and unset the global
   associative array variable.

2. If the option `add-bash-wrapper` is defined, a file `NAME.bash`
   is included which provides an entry point for Bash users to load the
   plugin.

3. A directory `.github/workflows` and a Github Actions script named
   `shell.yml` to automate shellcheck and shellspec. Generation can be
   skipped if the `no-github-dir` option is checked or both the options
   `no-shell-check` and `no-shell-spec` are set as the workflow has
   nothing to do.

4. A directory `functions` with an example autoloaded function
   named `NAME_example`. Generation can be skipped if the
   `no-functions-dir` option is set.

5. If the option `add-bin-dir is set, an empty `bin` directory for
   plugin specific binaries is created.

6. A file `.gitignore`. Generation can be  skipped if both the options
   `no-shell-check` and `no-shell-spec`.

7. A file `Makefile` for GNU Make. Generation can be  skipped if both
   the options `no-shell-check` and `no-shell-spec`.

8. A file `README.md` containing only a basic skeleton.

Rather than setting all options manually, three templates are provided with
pre-defined selection of settings. These are described in the table below.

| Feature / Template  | minimal | simple | complete |
| ------------------- | ------- | ------ | -------- |
| `add-bin-dir`       | false   | false  | true     |
| `add-bash-wrapper`  | false   | false  | true     |
| `no-aliases`        | true    | false  | false    |
| `no-functions-dir`  | true    | true   | false    |
| `no-git-init`       | true    | false  | false    |
| `no-github-dir`     | true    | true   | false    |
| `no-shell-check`    | true    | false  | false    |
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
  │  └─ workflows/           #   no-shell-check AND no-shell-spec
  │     └─ shell.yml
  ├─ bin/                    # when   add-bin-dir
  │  └─ .gitkeep
  ├─ functions/              # unless no-functions-dir
  │  └─ containers_example
  ├─ .git                    # unless no-git-init
  ├─ .gitignore              # unless no-git-init
  ├─ Makefile                # unless no-shell-check AND no-shell-spec
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
