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
# * **{{ plugin_var }}**: Plugin-defined global associative array with the following keys:
#   * **_NAME**: The name of this plugin.
#   * **_PATH**: The complete file path to the plugin's file.
#   * **_CONTEXT**: The plugin's state context.
#
# ### Public Variables
#
# * **{{ plugin_var }}_EXAMPLE**: if set it does something magical.
#

############################################################################
# @section Setup
# @description Standard path and variable setup.
#

typeset -A {{ plugin_var }}
{{ plugin_var }}[_PATH]="$(@zplugins_normalize_zero "$0")"
{{ plugin_var }}[_CONTEXT]="$(@zplugins_plugin_context {{ plugin_name }})"

############################################################################
# @section Lifecycle
# @description Plugin lifecycle functions.
#

#
# @description
#
# TBD.
#
# @noargs
#
{{ plugin_name }}_plugin_init() {
    builtin emulate -L zsh

    # Removing path/fpath entries.
    # @zplugins_add_to_path {{ plugin_name }} <PATH>
    # @zplugins_add_to_fpath {{ plugin_name }} <PATH>

    # Export any additional environment variables here.
    # @zplugins_save_global {{ plugin_name }} {{ plugin_var }}_EXAMPLE

    # Define any aliases here, or in their own section below.
    # @zplugins_define_alias {{ plugin_name }} <NAME> <EXPANSION>
}

#
# @description
#
# Called when the plugin is unloaded to clean up after itself.
#
# @noargs
#
{{ plugin_name }}_plugin_unload() {
    builtin emulate -L zsh

    # Reset global environment variables.
    #  @zplugins_restore_global {{ plugin_name }} {{ plugin_var }}_EXAMPLE

    unset {{ plugin_var }}
}

############################################################################
# @section Public
# @description Public functions, aliases, and varibles.
#

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
@zplugins_remember_fn {{ plugin_name }} {{ plugin_name }}_example
{%- endif %}

{% if include_aliases -%}
# Alias my_example ...
@zplugins_define_alias {{ plugin_name }} my_example '{{ plugin_name }}_example'
{%- endif %}
