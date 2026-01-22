# -*- mode: sh; eval: (sh-set-shell "zsh") -*-
#
# @name {{ plugin_display_name }}
{% if short_description -%}
# @brief {{ short_description }}
{% endif -%}
# @repository https://github.com/{{ github_user }}/zsh-{{ plugin_name }}-plugin
# @homepage **include if different from repository URL**
# @version **use semantic versioning, e.g. 0.1.0, or remove**
# @license **use license expressions, e.g., MIT AND Apache-2.0, or remove**
# @copyright **copyright notice in lieu of license, e.g., ©️ YEAR FULL_NAME <EMAIL>, or remove**
#
# @description
#
# Long description TBD.
#
# ### State Variables
#
# * **PLUGIN**: Plugin-defined global associative array with the following keys:
#   * **_NAME**: The name of this plugin.
#   * **_PATH**: The complete file path to the plugin's file.
#   * **_CONTEXT**: The plugin's state context.
#
# ### Public Variables
#
# * **{ plugin_var }}_EXAMPLE**: if set it does something magical.
#

############################################################################
# @section Setup
# @description Standard path and variable setup.
#

typeset -A PLUGIN
PLUGIN[_PATH]="$(@zplugins_normalize_zero "$0")"
PLUGIN[_NAME]="${${PLUGIN[_PATH]:t}%%.*}"
PLUGIN[_CONTEXT]="$(@zplugins_plugin_context ${PLUGIN[_NAME]})"

############################################################################
# @section Lifecycle
# @description Plugin lifecycle functions.
#

#
# @description
#
# This function does the initialization of variables in the global variable
# `{{ plugin_var }}`. It also adds to `path` and `fpath` as necessary.
#
# @noargs
#
{{ plugin_name }}_plugin_init() {
    builtin emulate -L zsh
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    # Removing path/fpath entries.
    # @zplugin_add_to_path ${PLUGIN[_NAME]} <PATH>
    # @zplugin_add_to_fpath ${PLUGIN[_NAME]} <PATH>

    # Export any additional environment variables here.
    #  @zplugin_save_global {{ plugin_name }} <VAR_NAME>

    # Define any aliases here, or in their own section below.
    # @zplugin_define_alias "${PLUGIN[_NAME]}" <NAME> <EXPANSION>

    # This should be the LAST step.
    @zplugin_register "${PLUGIN[_NAME]}" "${PLUGIN[_PATH]}"
}
@zplugin_remember_fn {{ plugin_name }}_plugin_init

#
# @description
#
# Called when the plugin is unloaded to clean up after itself.
#
# @noargs
#
{{ plugin_name }}_plugin_unload() {
    # See https://wiki.zshell.dev/community/zsh_plugin_standard#unload-function
    builtin emulate -L zsh

    # This should be the FIRST step.
    @zplugin_unregister "${PLUGIN[_NAME]}"

    # Removing path/fpath entries.
    # @zplugin_remove_from_path ${PLUGIN[_NAME]} <PATH>
    # @zplugin_remove_from_fpath ${PLUGIN[_NAME]} <PATH>

    # Reset global environment variables.
    #  @zplugin_restore_global ${PLUGIN[_NAME]} <VAR_NAME>

    # This should be the LAST step.
    unfunction {{ plugin_name }}_plugin_unload
}

############################################################################
# @section Public
# @description Public functions, aliases, and varibles.
#

# Initialize ${{{ plugin_var }}_EXAMPLE}

{% if not include_functions_dir -%}
#
# @description Some function that does some thing.
#
# @noargs
#
{{ plugin_name }}_example() {
    builtin emulate -L zsh

    printf "An example function in {{plugin_name}}, var: {{ _shv_start }}{{ plugin_var }}_EXAMPLE{{ _shv_end }}"
}
@zplugin_remember_fn "${PLUGIN[_NAME]}" {{ plugin_name }}_example
{%- endif %}

{% if include_aliases -%}
# Alias my_example ...
@zplugin_define_alias "${PLUGIN[_NAME]}" my_example '{{ plugin_name }}_example'
{%- endif %}

############################################################################
# @section Initialization
# @description Final plugin initialization.
#

{{ plugin_name }}_plugin_init

true
