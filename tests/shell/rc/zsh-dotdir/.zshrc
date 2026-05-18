# Minimal interactive zsh startup used by tests/shell/run.
# Loaded via `ZDOTDIR=<this-dir> zsh --no-globalrcs -i`.

PROMPT='NAVIPROMPT> '
PS2='> '

if [ -n "${SHELL_TESTS_PLUGIN:-}" ] && [ -f "$SHELL_TESTS_PLUGIN" ]; then
   source "$SHELL_TESTS_PLUGIN"
fi
