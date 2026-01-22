#!/usr/bin/env zsh # -*- mode: sh; eval: (sh-set-shell "zsh") -*-

if ! command -v shdoc 2>&1 >/dev/null; then
    echo "shdoc not installed, go to https://github.com/reconquest/shdoc"
    exit 1
fi

PLUGIN_NAME={{ module_name }}
INPUT_PATH=${${PWD:-.}:P}
OUTPUT_PATH=${INPUT_PATH}/doc
MODULE_PATH=${INPUT_PATH}/${PLUGIN_NAME}

echo -n '.'
shdoc ${INPUT_PATH}/${PLUGIN_NAME}.plugin.zsh > ${OUTPUT_PATH}/index.md

if [[ -d ${MODULE_PATH} ]]; then
    for file in ${MODULE_PATH}/*.zsh; do
        echo -n '.'
        shdoc ${file} > ${OUTPUT_PATH}/${${file:t}:r}.md
    done
fi
echo ' done'