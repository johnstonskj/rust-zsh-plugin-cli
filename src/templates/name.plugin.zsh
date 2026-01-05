# -*- mode: sh; eval: (sh-set-shell "zsh") -*-
#
# Name: { plugin_display_name }}
# Repository: https://github.com/{{ github_user }}/zsh-{{ plugin_name }}-plugin
{% if short_description -%}
# Description:
#     {{ short_description }}
#
{% else %}
#
{%- endif %}

############################################################################
# Standard Setup Behavior
############################################################################

# See https://wiki.zshell.dev/community/zsh_plugin_standard#zero-handling
0="${ZERO:-${${0:#$ZSH_ARGZERO}:-${(%):-%N}}}"
0="${${(M)0:#/*}:-$PWD/$0}"

# See https://wiki.zshell.dev/community/zsh_plugin_standard#standard-plugins-hash
declare -gA {{ plugin_var }}
${{ plugin_var }}[_PLUGIN_DIR]="${0:h}"
{%- if include_aliases %}
${{ plugin_var }}[_ALIASES]=""
{%- endif %}
${{ plugin_var }}[_FUNCTIONS]=""

#
# Public variables:
#
# `{{ plugin_var }}_EXAMPLE`; if set it does something magical.
#

############################################################################
# Internal Support Functions
############################################################################

#
# This function will add to the `{{ plugin_var }}[_FUNCTIONS]` list which is
# used at unload time to `unfunction` plugin-defined functions.
#
_{{ plugin_name }}_remember_fn() {
    builtin emulate -L zsh

    local fn_name="${1}"
    if [[ -z "{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }}" ]]; then
        {{ plugin_var }}[_FUNCTIONS]="${fn_name}"
    elif [[ ",{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }}," != *",${fn_name},"* ]]; then
        {{ plugin_var }}[_FUNCTIONS]="{{ _shv_start }}{{ plugin_var }}[_FUNCTIONS]{{ _shv_end }},${fn_name}"
    fi
}
_{{ plugin_name }}_remember_fn _{{ plugin_name }}_remember_fn

{% if include_aliases -%}
_{{ plugin_name }}_define_alias() {
    local alias_name="${1}"
    local alias_value="${2}"

    alias ${alias_name}=${alias_value}

    if [[ -z "{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }}" ]]; then
        {{ plugin_var }}[_ALIASES]="${alias_name}"
    elif [[ ",{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }}," != *",${alias_name},"* ]]; then
        {{ plugin_var }}[_ALIASES]="{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }},${alias_name}"
    fi
}
_{{ plugin_name }}_remember_fn _{{ plugin_name }}_remember_alias
{%- endif %}

#
# This function does the initializtion of variables in the global variable
# `{{ plugin_var }}`. It also adds to `path` and `fpath` as necessary.
# This variable is an associative array with the following private keys:
#
# - \`_PLUGIN_DIR\`; the directory the plugin is sourced from.
# - \`_PLUGIN_BIN_DIR\`; the directory (if present) for plugin specific binaries.
# - \`_PLUGIN_FNS_DIR\`; the directory (if present) for plugin autoload functions.
# - \`_FUNCTIONS\`; a list of all functions defined by the plugin.
#
{{ plugin_name }}_plugin_init() {
    emulate -L zsh

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
            _{{ plugin_name }}_remember_fn ${fn}
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
}
_{{ plugin_name }}_remember_fn {{ plugin_name }}_plugin_init

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
    
    # Remove all remembered aliases.
    local aliases
    IFS=',' read -r -A aliases <<< "{{ _shv_start }}{{ plugin_var }}[_ALIASES]{{ _shv_end }}"
    local alias
    for alias in ${aliases[@]}; do
        unalias "${alias}"
    done

    # Remove the global data variable.
    unset {{ plugin_var }}

    {% if include_functions_dir -%}
    # Remove functions directory from fpath.
    fpath=("${(@)fpath:#{{ _shv_start }}{{ plugin_var }}[_PLUGIN_FNS_DIR]{{ _shv_end }}}")
    {%- endif -%}

    # Remove this function.
    unfunction {{ plugin_name }}_plugin_unload
}

{% if not include_functions_dir %}
############################################################################
# Public Functions
############################################################################

{{ plugin_name }}_example() {
    builtin emulate -L zsh

    printf "An example function in {{plugin_name}}, var: {{ _shv_start }}{{ plugin_var }}_EXAMPLE{{ _shv_end }}"
}
_{{ plugin_name }}_remember_fn {{ plugin_name }}_example
{% endif %}

{% if include_aliases -%}
############################################################################
# Plugin-defined Aliases
############################################################################

_{{ plugin_name }}_define_alias my_example '{{ plugin_name }}_example'
{%- endif %}

############################################################################
# Initialize Plugin
############################################################################

{{ plugin_name }}_plugin_init
true
