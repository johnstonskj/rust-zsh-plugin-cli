# -*- mode: sh; eval: (sh-set-shell "bash") -*-

emulate() {
    : # no-op
}

install_path() {
    local install_dir
    # shellcheck disable=SC2154
    if [[ -n "${ZSH_VERSION}" ]]; then
        install_dir="${funcsourcetrace[1]}"
    elif [[ -n "${BASH_VERSION}" ]]; then
        install_dir="${BASH_SOURCE[1]}"
    else
        echo "Error: not Zsh and not Bash = not supported."
        exit 1
    fi

    if [[ "${install_dir}" == */* ]]; then
        install_dir="${install_dir%/*}"
    else
        install_dir='.'
    fi

    if [[ -n "${ZSH_VERSION}" ]]; then
        install_dir="${install_dir:A}"
    elif [[ -n "${BASH_VERSION}" ]]; then
        install_dir=$(realpath "${install_dir}")
    else
        echo "Error: not Zsh and not Bash = not supported."
        exit 1
    fi
    printf '%s' "${install_dir}"
}

source "$(install_path)/{{ plugin_name }}.plugin.zsh"
unfunction install_path

{% if include_functions_dir -%}
if [[ -d "$(install_path)/functions" ]]; then
    for file in $(install_path)/functions/*; do
        source "${file}"
    done
fi
{%- endif %}