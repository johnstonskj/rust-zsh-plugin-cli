# -*- mode: sh; eval: (sh-set-shell "zsh") -*-
#
# Name: {{ plugin_display_name }}
{% if short_description -%}
# Description: {{ short_description }}
{% endif -%}
# Repository: https://github.com/{{ github_user }}/zsh-{{ plugin_name }}-plugin
# Homepage: **include if different from repository URL**
# Version: **use semantic versioning, e.g. 0.1.0, or remove**
# License: **use license expressions, e.g., MIT AND Apache-2.0, or remove**
# Copyright: **copyright notice in lieu of license, e.g., ©️ YEAR FULL_NAME <EMAIL>, or remove**
#
# Long description TBD.
#
# State variables:
#
# * `PLUGIN`; plugin-defined global associative array with the following keys:
{% if include_aliases -%}
#   * `_ALIASES`; a list of all aliases defined by the plugin.
{% endif -%}
#   * `_FUNCTIONS`; a list of all functions defined by the plugin.
#   * `_PLUGIN_DIR`; the directory the plugin is sourced from.
#   * `_PLUGIN_FILE`; the file in _PLUGIN_DIR the plugin is sourced from.
{% if include_bin_dir -%}
#   * `_PLUGIN_BIN_DIR`; the directory (if present) for plugin specific binaries.
{% endif -%}
{% if include_functions_dir -%}
#   * `_PLUGIN_FNS_DIR`; the directory (if present) for plugin autoload functions.
{% endif -%}
#
# Public Variables:
#
# * `{{ plugin_var }}_EXAMPLE`; if set it does something magical.
#

############################################################################
# Plugin Setup
############################################################################

typeset -A PLUGIN
PLUGIN[_PATH]="$(@zplugins_normalize_zero "$0")"
PLUGIN[_NAME]="${${PLUGIN[_PATH]:t}%%.*}"
PLUGIN[_CONTEXT]="$(@zplugins_plugin_context ${PLUGIN[_NAME]})"

############################################################################
# Plugin Lifecycle
############################################################################

#
# This function does the initialization of variables in the global variable
# `{{ plugin_var }}`. It also adds to `path` and `fpath` as necessary.
#
{{ plugin_name }}_plugin_init() {
    builtin emulate -L zsh
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    # Export any additional environment variables here.
    #  @zplugin_save_global {{ plugin_name }} <VAR_NAME>

    # Define any aliases here, or in their own section below.

    # This should be the LAST step.
    @zplugin_register {{ plugin_name }} ${PLUGIN[_PATH]}
}
@zplugin_remember_fn {{ plugin_name }}_plugin_init

{{ plugin_name }}_plugin_unload() {
    # See https://wiki.zshell.dev/community/zsh_plugin_standard#unload-function
    builtin emulate -L zsh

    # This should be the FIRST step.
    @zplugin_unregister {{ plugin_name }}

    # Removing path/fpath entries.
    # path=( "${(@)path:#{{ _shv_start }}{{ plugin_var }}[_PATH]{{ _shv_end }}}" )

    # Reset global environment variables.
    #  @zplugin_restore_global {{ plugin_name }} <VAR_NAME>

    # This should be the LAST step.
    unfunction {{ plugin_name }}_plugin_unload
}

############################################################################
# Plugin Public Things
############################################################################

{% if not include_functions_dir -%}
{{ plugin_name }}_example() {
    builtin emulate -L zsh

    printf "An example function in {{plugin_name}}, var: {{ _shv_start }}{{ plugin_var }}_EXAMPLE{{ _shv_end }}"
}
@zplugin_remember_fn {{ plugin_name }} {{ plugin_name }}_example
{%- endif %}

{% if include_aliases -%}
@zplugin_define_alias {{ plugin_name }} my_example '{{ plugin_name }}_example'
{%- endif %}

############################################################################
# Plugin Initialization
############################################################################

{{ plugin_name }}_plugin_init

true
