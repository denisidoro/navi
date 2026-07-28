# Minimal interactive fish startup used by tests/shell/run.
#
# Loaded via: fish ... -C "source <this-file>"
#
# Requires SHELL_TESTS_PLUGIN in the environment (set by tests/shell/lib.bash).

function fish_prompt
    echo -n "$SHELL_TESTS_PROMPT_TEXT"
end

if test -n "$SHELL_TESTS_PLUGIN"
    source "$SHELL_TESTS_PLUGIN"
end
