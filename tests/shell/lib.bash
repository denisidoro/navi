#!/usr/bin/env bash
# vim: filetype=sh
#
# tmux-driven helpers used by tests/shell/run to exercise the
# shell plugins (bash / zsh / fish) end-to-end.
#
# Design:
#  - Each test spawns a fresh, named tmux session running the target
#    shell with the matching `shell/navi.plugin.*` sourced.
#  - The plugin invokes navi against a tiny deterministic cheat file
#    in tests/shell/cheats, so `--best-match` returns the same snippet
#    every time (no live fzf interaction required, except for bash
#    which always opens fzf — we accept the only match with Enter).
#  - Snippets, when executed, print known `NAVI_TEST::...::END`
#    sentinels. Tests assert on captured pane contents instead of
#    poking shell-specific editor buffers.

readonly SHELL_TESTS_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly SHELL_TESTS_CHEATS_PATH="${SHELL_TESTS_HOME}/cheats"
readonly SHELL_TESTS_RC_DIR="${SHELL_TESTS_HOME}/rc"

# Default polling timeout (seconds) for shell::wait_for. tmux + navi
# need a beat to start, especially on cold CI runners.
readonly SHELL_TESTS_WAIT_TIMEOUT="${SHELL_TESTS_WAIT_TIMEOUT:-15}"

# A prompt marker injected by tests/shell/rc/* so shell::wait_for_prompt
# is deterministic regardless of the runner's $PS1 / $PROMPT defaults.
readonly SHELL_TESTS_PROMPT_MARKER='NAVIPROMPT> '

shell::_session_name() {
   local -r shell="$1"
   local -r case_id="$2"
   echo "navi_shell_test_${shell}_${case_id}_$$"
}

shell::_plugin_path() {
   local -r shell="$1"
   case "$shell" in
      bash) echo "${NAVI_HOME}/shell/navi.plugin.bash" ;;
      zsh)  echo "${NAVI_HOME}/shell/navi.plugin.zsh" ;;
      fish) echo "${NAVI_HOME}/shell/navi.plugin.fish" ;;
      *) return 1 ;;
   esac
}

# Build the env-prefix tmux launches the shell with. Same for all
# three shells; the rc/init files do the prompt setup and source the
# plugin.
shell::_env_prefix() {
   local -r shell="$1"
   local -r plugin="$(shell::_plugin_path "$shell")"
   local -r navi_dir="$(dirname "$NAVI_EXE")"

   printf '%s' \
      "env " \
      "PATH='${navi_dir}:${PATH}' " \
      "TERM='${TERM:-xterm-256color}' " \
      "NAVI_CONFIG='${NAVI_HOME}/tests/config.yaml' " \
      "NAVI_PATH='${SHELL_TESTS_CHEATS_PATH}' " \
      "SHELL_TESTS_PLUGIN='${plugin}' "
}

shell::_launch_cmd() {
   local -r shell="$1"
   local -r prefix="$(shell::_env_prefix "$shell")"

   case "$shell" in
      bash)
         printf '%s%s' "$prefix" \
            "bash --noprofile --rcfile '${SHELL_TESTS_RC_DIR}/bashrc' -i"
         ;;
      zsh)
         printf '%s%s%s' "$prefix" \
            "ZDOTDIR='${SHELL_TESTS_RC_DIR}/zsh-dotdir' " \
            "zsh --no-globalrcs -i"
         ;;
      fish)
         local -r plugin="$(shell::_plugin_path "$shell")"
         local -r init_cmd="function fish_prompt; echo -n \"${SHELL_TESTS_PROMPT_MARKER}\"; end; source \"${plugin}\""
         printf "%sfish --no-config -i -C '%s'" "$prefix" "$init_cmd"
         ;;
   esac
}

# Capture the full visible pane contents, including scrollback.
shell::pane() {
   local -r session="$1"
   tmux capture-pane -t "$session" -p -S - 2>/dev/null || true
}

# Poll the pane until `pattern` (extended regex) appears, or the
# timeout (in seconds) elapses. Returns 0 on match, 1 on timeout.
shell::wait_for() {
   local -r session="$1"
   local -r pattern="$2"
   local -r timeout="${3:-$SHELL_TESTS_WAIT_TIMEOUT}"
   local -r deadline=$(( $(date +%s) + timeout ))

   while [ "$(date +%s)" -lt "$deadline" ]; do
      if shell::pane "$session" | grep -Eq "$pattern"; then
         return 0
      fi
      sleep 0.2
   done

   return 1
}

shell::wait_for_prompt() {
   shell::wait_for "$1" "$SHELL_TESTS_PROMPT_MARKER" "${2:-}"
}

# Start a fresh tmux session for the given shell. Returns the session
# name on stdout; non-zero on launch failure.
shell::start() {
   local -r shell="$1"
   local -r case_id="$2"
   local -r session="$(shell::_session_name "$shell" "$case_id")"
   local -r launch_cmd="$(shell::_launch_cmd "$shell")"

   tmux kill-session -t "$session" 2>/dev/null || true
   tmux new-session -d -s "$session" -x 200 -y 50 "$launch_cmd"

   if ! shell::wait_for_prompt "$session"; then
      log::error "Shell '$shell' failed to reach a prompt in session '$session'"
      log::error "Launch command was: $launch_cmd"
      log::error "Pane contents:"
      shell::pane "$session" 1>&2
      tmux kill-session -t "$session" 2>/dev/null || true
      return 1
   fi

   echo "$session"
}

shell::stop() {
   local -r session="$1"
   tmux kill-session -t "$session" 2>/dev/null || true
}

# Literal typing (no key interpretation). Safe for snippet text that
# may contain backslashes, quotes, dashes, etc.
shell::type() {
   local -r session="$1"
   local -r text="$2"
   tmux send-keys -t "$session" -l -- "$text"
}

# Send a tmux key name (e.g. Enter, Escape, C-g, C-c). Multiple keys
# may be passed; each is forwarded as its own send-keys argument.
shell::keys() {
   local -r session="$1"
   shift
   tmux send-keys -t "$session" "$@"
}

# Sends the plugin's bound trigger key. All three target shells use
# Ctrl-G; if a follow-up PR adds elvish (which uses Alt-h) it can
# branch here.
shell::trigger() {
   local -r session="$1"
   tmux send-keys -t "$session" 'C-g'
}

shell::enter() {
   local -r session="$1"
   tmux send-keys -t "$session" 'Enter'
}

shell::escape() {
   local -r session="$1"
   tmux send-keys -t "$session" 'Escape'
}

# Convenience: kill any stray tmux sessions left behind by aborted
# runs. Mirrors `_kill_tmux` in tests/run.
shell::kill_all() {
   pkill -f "tmux.*navi_shell_test_.*_$$" 2>/dev/null || true
}
