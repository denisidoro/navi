#!/usr/bin/env bash

ui::pick() {
   fzf --height '100%' --inline-info "$@"
}

ui::select() {
   local -r cheats="$1"

   echo "$cheats" \
      | cheat::read_many \
      | ui::pick -i --ansi \
      | selection::standardize
}

ui::clear_previous_line() {
   tput cuu1 && tput el || true
}
