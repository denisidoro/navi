#!/usr/bin/env bash

ui::pick() {
   fzf --height '100%' --inline-info "$@"
}

ui::select() {
   local readonly cheats="$1"
   local readonly script_path="$(which navi | head -n1 || echo "${SCRIPT_DIR}/navi")"
   local readonly preview_cmd="echo {} | tr ' ' '^' | xargs -I% \"${script_path}\" --command-for %"

   echo "$cheats" \
      | cheat::read_many \
      | ui::pick -i \
      	--ansi \
         $($preview && echo "--preview") \
         "$($preview && echo "$preview_cmd")" \
      	--preview-window 'up:1' \
      | selection::standardize
}

ui::clear_previous_line() {
   tput cuu1 && tput el || true
}
