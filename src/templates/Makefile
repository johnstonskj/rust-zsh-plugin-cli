# -*- mode: makefile-gmake -*-

SOURCES={{ plugin_name }}.plugin.zsh{% if add_bash_wrapper %} {{ plugin_name }}.bash{% endif %}{% if include_functions_dir %} functions/*{% endif %}
{% if include_shell_spec %}TEST_SOURCES=spec/{{ plugin_name }}_spec.sh spec/spec_helper.sh{% endif -%}

all: check test

check:{% if include_shell_check %} check_sources{% endif %}{% if include_shell_spec %} check_test_sources{% endif %}

{% if include_shell_check -%}
check_sources: $(SOURCES)
	shellcheck --check-sourced --color=auto --shell=bash $^
{% endif %}
{% if include_shell_spec -%}
check_test_sources: $(TEST_SOURCES)
	shellspec --syntax-check

test: $(SOURCES) $(TEST_SOURCES)
	shellspec --shell /opt/homebrew/bin/zsh --format documentation --output junit
	shellspec --shell /opt/homebrew/bin/bash --format documentation --output junit

coverage: $(SOURCES) $(TEST_SOURCES)
	shellspec --kcov --kcov-options "--include-pattern=.sh,.bash,.zsh"
{% endif %}