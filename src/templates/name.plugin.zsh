# -*- mode: sh; eval: (sh-set-shell "zsh") -*-
#
# Plugin Name: {{ plugin_display_name }}
# Repository: https://github.com/{{ github_user }}/zsh-{{ plugin_name }}-plugin
#
{% if short_description -%}
# Description:
#
#   {{ short_description }}
#
{% endif -%}
# Public variables:
#
# * `{{ plugin_var }}`; plugin-defined global associative array with the following keys:
{% if include_aliases -%}
#   * `_ALIASES`; a list of all aliases defined by the plugin.
{% endif -%}
#   * `_FUNCTIONS`; a list of all functions defined by the plugin.
#   * `_PLUGIN_DIR`; the directory the plugin is sourced from.
{% if include_bin_dir -%}
#   * `_PLUGIN_BIN_DIR`; the directory (if present) for plugin specific binaries.
{% endif -%}
{% if include_functions_dir -%}
#   * `_PLUGIN_FNS_DIR`; the directory (if present) for plugin autoload functions.
{% endif -%}
# * `{{ plugin_var }}_EXAMPLE`; if set it does something magical.
#

############################################################################
# Standard Setup Behavior
############################################################################

# See https://wiki.zshell.dev/community/zsh_plugin_standard#zero-handling
0="${ZERO:-${${0:#$ZSH_ARGZERO}:-${(%):-%N}}}"
0="${${(M)0:#/*}:-$PWD/$0}"

# See https://wiki.zshell.dev/community/zsh_plugin_standard#standard-plugins-hash
declare -gA {{ plugin_var }}
{{ plugin_var }}[_PLUGIN_DIR]="${0:h}"
{%- if include_aliases %}
{{ plugin_var }}[_ALIASES]=""
{%- endif %}
{{ plugin_var }}[_FUNCTIONS]=""

# Set the path for any custom directories here.
# Example:
# {{ plugin_var }}[_PATH]="<SOME_PATH>"

# Saving the current state for any modified global environment variables.
# Example:
# {{ plugin_var }}[_OLD_<VAR_NAME>]="${<VAR_NAME>}"

############################################################################
# Internal Support Functions
############################################################################

#
# This function will add to the `{{ plugin_var }}[_FUNCTIONS]` list which is
# used at unload time to `unfunction` plugin-defined functions.
#
# See https://wiki.zshell.dev/community/zsh_plugin_standard#unload-function
# See https://wiki.zshell.dev/community/zsh_plugin_standard#the-proposed-function-name-prefixes
#
.{{ plugin_name }}_remember_fn() {
    builtin emulate -L zsh

    local fn_name="${1}"
    if [[ -z "{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }}" ]]; then
        {{ plugin_var }}[_FUNCTIONS]="${fn_name}"
    elif [[ ",{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }}," != *",${fn_name},"* ]]; then
        {{ plugin_var }}[_FUNCTIONS]="{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }},${fn_name}"
    fi
}
.{{ plugin_name }}_remember_fn .{{ plugin_name }}_remember_fn

{% if include_aliases -%}
.{{ plugin_name }}_define_alias() {
    local alias_name="${1}"
    local alias_value="${2}"

    alias ${alias_name}=${alias_value}

    if [[ -z "{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }}" ]]; then
        {{ plugin_var }}[_ALIASES]="${alias_name}"
    elif [[ ",{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }}," != *",${alias_name},"* ]]; then
        {{ plugin_var }}[_ALIASES]="{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }},${alias_name}"
    fi
}
.{{ plugin_name }}_remember_fn .{{ plugin_name }}_remember_alias
{%- endif %}

#
# This function does the initialization of variables in the global variable
# `{{ plugin_var }}`. It also adds to `path` and `fpath` as necessary.
#
{{ plugin_name }}_plugin_init() {
    builtin emulate -L zsh
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    {% if include_functions_dir -%}
    # See https://wiki.zshell.dev/community/zsh_plugin_standard#functions-directory
    if [[ -d "{{ _shv_start }}{{ plugin_var }}[_PLUGIN_DIR]{{ _shv_end }}/functions" ]]; then
        {{ plugin_var }}[_PLUGIN_FNS_DIR]="{{ _shv_start }}{{ plugin_var }}[_PLUGIN_DIR]{{ _shv_end }}/functions"

        if [[ $PMSPEC != *f* ]]; then
            # For compliant plugin managers
            fpath+=( "{{ _shv_start }}{{ plugin_var }}[_PLUGIN_FNS_DIR]{{ _shv_end }}" )
        elif [[ ${zsh_loaded_plugins[-1]} != */{{ plugin_name }} && -z ${fpath[(r){{ _shv_start }}{{ plugin_var }}[_PLUGIN_FNS_DIR]{{ _shv_end }}]} ]]; then
            # For non-compliant plugin managers
            fpath+=( "{{ _shv_start }}{{ plugin_var }}[_PLUGIN_FNS_DIR]{{ _shv_end }}" )
        fi

        local fn
        for fn in {{ _shv_start }}{{ plugin_var }}[_PLUGIN_FNS_DIR]{{ _shv_end }}/*(.:t); do
            autoload -Uz ${fn}
            .{{ plugin_name }}_remember_fn ${fn}
        done

    fi
    {%- endif %}

    {% if include_bin_dir -%}
    # See https://wiki.zshell.dev/community/zsh_plugin_standard#binaries-directory
    if [[ -d "{{ _shv_start }}{{ plugin_var }}[_PLUGIN_DIR]{{ _shv_end }}/bin" ]]; then
        {{ plugin_var }}[_PLUGIN_BIN_DIR]="{{ _shv_start }}{{ plugin_var }}[_PLUGIN_DIR]{{ _shv_end }}/bin"

        if [[ $PMSPEC != *b* ]]; then
            # For compliant plugin managers
            path+=( "{{ plugin_var }}[_PLUGIN_BIN_DIR]" )
        elif [[ ${zsh_loaded_plugins[-1]} != */{{ plugin_name }} && -z ${fpath[(r){{ plugin_var }}[_PLUGIN_BIN_DIR]]} ]]; then
            # For non-compliant plugin managers
            path+=( "{{ plugin_var }}[_PLUGIN_BIN_DIR]" )
        fi
    fi
    {%- endif %}

    # Add _PATH to path.
    # path+=( "{{ plugin_var }}[_PATH]" )

    # Export environment variables.

    # Define any aliases here, or in their own section below.
}
.{{ plugin_name }}_remember_fn {{ plugin_name }}_plugin_init

############################################################################
# Plugin Unload Function
############################################################################

# See https://wiki.zshell.dev/community/zsh_plugin_standard#unload-function
{{ plugin_name }}_plugin_unload() {
    builtin emulate -L zsh

    # Remove all remembered functions.
    local plugin_fns
    IFS=',' read -r -A plugin_fns <<< "{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }}"
    local fn
    for fn in ${plugin_fns[@]}; do
        whence -w "${fn}" &> /dev/null && unfunction "${fn}"
    done
    
    {% if include_aliases %}
    # Remove all remembered aliases.
    local aliases
    IFS=',' read -r -A aliases <<< "{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }}"
    local alias
    for alias in ${aliases[@]}; do
        unalias "${alias}"
    done
    {% endif %}

    {% if include_bin_dir -%}
    # Remove bin directory from path.
    path=( "${(@)path:#{{ _shv_start }}{{ plugin_var }}[_PLUGIN_BIN_DIR]{{ _shv_end }}}" )
    {%- endif -%}
    {% if include_functions_dir -%}
    # Remove functions directory from fpath.
    fpath=( "${(@)fpath:#{{ _shv_start }}{{ plugin_var }}[_PLUGIN_FNS_DIR]{{ _shv_end }}}" )
    {%- endif -%}

    # Removing path/fpath entries.
    # Example:
    # path=( "${(@)path:#{{ _shv_start }}{{ plugin_var }}[_PATH]{{ _shv_end }}}" )

    # Reset global environment variables .
    # Example:
    # export <VAR_NAME>="{{ _shv_start }}{{ plugin_var }}[_OLD_<VAR_NAME>]{{ _shv_end }}"

    # Remove the global data variable (after above!).
    unset {{ plugin_var }}

    # Remove this function last.
    unfunction {{ plugin_name }}_plugin_unload
}

{% if not include_functions_dir -%}
############################################################################
# Public Functions
############################################################################

{{ plugin_name }}_example() {
    builtin emulate -L zsh

    printf "An example function in {{plugin_name}}, var: {{ _shv_start }}{{ plugin_var }}_EXAMPLE{{ _shv_end }}"
}
.{{ plugin_name }}_remember_fn {{ plugin_name }}_example
{%- endif %}

{% if include_aliases -%}
############################################################################
# Plugin-defined Aliases
############################################################################

.{{ plugin_name }}_define_alias my_example '{{ plugin_name }}_example'
{%- endif %}

############################################################################
# Initialize Plugin
############################################################################

{{ plugin_name }}_plugin_init

true
