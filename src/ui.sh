#!/usr/bin/env bash

ui::pick() {
   local -r autoselect="$(dict::get "$OPTIONS" autoselect)"

   local args=()
   if ${autoselect:-false}; then
      args+=("--select-1")
   fi

   fzf --height '100%' "${args[@]:-}" --inline-info "$@"
}

# TODO: separation of concerns
ui::select() {
   local -r cheats="$1"

   local -r script_path="${SCRIPT_DIR}/navi"
   local -r preview_cmd="echo \"{}\" | tr ' ' '${ARG_DELIMITER}' | xargs -I% \"${script_path}\" preview %"

   local -r query="$(dict::get "$OPTIONS" query)"
   local -r entry_point="$(dict::get "$OPTIONS" entry_point)"
   local -r preview="$(dict::get "$OPTIONS" preview)"

   local args=()
   args+=("-i")
   args+=("--ansi")
   if $preview; then
      args+=("--preview"); args+=("$preview_cmd")
      args+=("--preview-window"); args+=("up:1")
   fi
   if [ -n "$query" ]; then
      args+=("--query=${query} ")
   fi
   if [ "$entry_point" = "search" ]; then
      args+=("--header")
      args+=("Displaying online results. Please refer to 'navi --help' for details")
   fi

   echo "$cheats" \
      | cheat::read_many \
      | ui::pick "${args[@]}" \
      | selection::dict
}

ui::clear_previous_line() {
   tput cuu1 && tput el || true
}
