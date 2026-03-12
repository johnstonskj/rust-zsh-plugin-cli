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
# * **{{ plugin_var }}_PATH**: The absolute path to the plugin's file.
#
# ### Public Variables
#
# * **{{ plugin_var }}_EXAMPLE**: if set it does something magical.
#

###################################################################################################
# @section Setup
# @description Standard path and variable setup.
#

{{ plugin_var }}_PLUGIN_PATH="$(@zplugins_normalize_zero "$0")"

###################################################################################################
# @section Lifecycle
# @description Plugin lifecycle functions.
#

# Declare any dependencies here, it needs to be done BEFORE the plugin manager calls the plugin
# _init function. In this case, you now have access to logging functions.
@zplugins_declare_plugin_dependencies {{ plugin_name }} shlog

#
# @description
#
# Called when the plugin is loaded, allows for additional actions beyond those performed by
# the plugin manager.
#
# @noargs
#
{{ plugin_name }}_plugin_init() {
    builtin emulate -L zsh
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    # Removing any non-standard path/fpath entries.
    # @zplugin_add_to_path {{ plugin_name }} <PATH>
    # @zplugin_add_to_fpath {{ plugin_name }} <PATH>

    # Save, and set, any public environment variables here.
    @zplugins_envvar_save {{ plugin_name }} {{ plugin_var }}_EXAMPLE
    typeset -g {{ plugin_var }}_EXAMPLE={{ _shv_start }}{{ plugin_var }}_EXAMPLE:-1{{ _shv_end }}

    # Define any aliases here, or in their own section below.
    # @zplugin_define_alias {{ plugin_name }} <NAME> '<EXPANSION>'
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

    # Reset any public environment variables.
    @zplugins_envvar_restore {{ plugin_name }} {{ plugin_var }}_EXAMPLE

    unset {{ plugin_var }}_PLUGIN_PATH
}

###################################################################################################
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
@zplugin_remember_fn {{ plugin_name }} {{ plugin_name }}_example
{%- endif %}

{% if include_aliases -%}
# Define any aliases here, or in the plugin _init function above.
@zplugin_define_alias {{ plugin_name }} my_example '{{ plugin_name }}_example'
{%- endif %}
