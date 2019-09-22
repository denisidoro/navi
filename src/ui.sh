#!/usr/bin/env bash

ui::pick() {
   fzf --height '100%' --inline-info "$@"
}

ui::select() {
   local readonly cheats="$1"
   local readonly script_path="$(which navi | head -n1 || echo "${SCRIPT_DIR}/navi")"
   local readonly preview_cmd="echo \"{}\" | tr ' ' '^' | xargs -I% \"${script_path}\" preview %"

   local args=()
   args+=("-i")
   args+=("--ansi")
   if $preview; then
      args+=("--preview"); args+=("$preview_cmd")
      args+=("--preview-window"); args+=("up:1")
   fi
   if [ -n "${query:-}" ]; then
      args+=("--query=${query} ")
   fi
   if [ "${entry_point:-}" = "search" ]; then
      args+=("--header")
      args+=("Displaying online results. Please refer to 'navi --help' for details")
   fi

   echo "$cheats" \
      | cheat::read_many \
      | ui::pick "${args[@]}" \
      | selection::standardize
}

ui::clear_previous_line() {
   tput cuu1 && tput el || true
}
