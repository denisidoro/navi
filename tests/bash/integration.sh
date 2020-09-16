#!/usr/bin/env bash
set -euo pipefail

source "${NAVI_HOME}/scripts/aux/tests.sh"

_kill_tmux() {
   pkill -f tmux 2>/dev/null || true
}

_assert_tmux() {
   local -r log_file="$1"
   local -r sessions="$(tmux list-sessions)"
   if [ -z "$sessions" ]; then
      _kill_tmux
      cat "$log_file"
      return 1
   fi
}

_integration() {
   _kill_tmux
   local -r log_file="${NAVI_HOME}/target/ci.log"
   local -r cheats_path="$(navi info cheats-path)"
   mkdir -p "$cheats_path" 2>/dev/null || true
   local -r bak_cheats_path="$(mktemp -d "${cheats_path}_XXXXX")"
   rm "$log_file" 2>/dev/null || true
   mv "$cheats_path" "$bak_cheats_path" 2>/dev/null || true

   echoerr "Starting sessions..."
   tmux new-session -d -s ci "unset NAVI_PATH; navi |& tee '${log_file}'"
   sleep 5
   _assert_tmux "$log_file"

   echoerr "Downloading default cheatsheets..."
   tmux send-key -t ci "download default"; tmux send-key -t ci "Enter"
   sleep 1
   _assert_tmux "$log_file"

   echoerr "Confirming import..."
   tmux send-key -t ci "y"; tmux send-key -t ci "Enter"
   sleep 6
   _assert_tmux "$log_file"

   echoerr "Running snippet..."
   tmux send-key -t ci "pwd"
   sleep 1  
   tmux send-key -t ci "Enter"

   sleep 2
   cat "$log_file" | tail -n10 | grep -q "/navi"
}

test::set_suite "integration"
test::run "welcome->pwd" _integration
