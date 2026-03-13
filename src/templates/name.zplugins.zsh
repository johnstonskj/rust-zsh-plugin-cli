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
# ### Public Variables
#
# * **{{ plugin_var }}_EXAMPLE**: if set it does something magical.
#

typeset -gi EC_SUCCESS

###################################################################################################
# @section Globals
# @description
#
# Import any globals from other plugins (using `typeset -g`) and initialize any plugin globals
# using either `declare` or `declare -g` for exported values.
#
# ### Internal Variables
#
# * **{{ plugin_var }}_PLUGIN_PATH**: The complete file path to the plugin's file.
#

declare -g { plugin_var }}_EXAMPLE

declare {{ plugin_var }}_PLUGIN_PATH="$(@zplugins_normalize_zero "$0")"

###################################################################################################
# @section Lifecycle
# @description
#
# Plugin core lifecycle components.
#
# 1. Declare any dependencies here, it needs to be done **before** the plugin manager calls the plugin's `_init` function.
# 2. Declare the function `{{ plugin_name }}_plugin_init` to perform any special initialization, this may not be necessary.
# 3. Declare the function `{{ plugin_name }}_plugin_unload` to perform any special clean-up, this may not be necessary.
#

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

    # Add any additional path/fpath entries.
    # @zplugins_add_to_path {{ plugin_name }} <PATH>
    # @zplugins_add_to_fpath {{ plugin_name }} <PATH>

    # Save, and set, any public environment variables here.
    @zplugins_envvar_save {{ plugin_name }} {{ plugin_var }}_EXAMPLE
    {{ plugin_var }}_EXAMPLE={{ _shv_start }}{{ plugin_var }}_EXAMPLE:-1{{ _shv_end }}

    # Define any aliases here.
    # @zplugins_define_alias {{ plugin_name }} <NAME> '<EXPANSION>''

    return ${EC_SUCCESS}
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

    # Unset any plugin-specific globals not saved with `@zplugins_envvar_save`.
    unset {{ plugin_var }}_PLUGIN_PATH

    return ${EC_SUCCESS}
}

###################################################################################################
# @section Public
# @description Public functions and aliases.
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
