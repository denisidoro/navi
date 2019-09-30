#!/usr/bin/env bash

# no-op hack to set dependency order resolution
dep() {
   :
}

command_exists() {
   type "$1" &> /dev/null
}

platform::existing_command() {
   for cmd in "$@"; do
      if command_exists "$cmd"; then
         echo "$cmd"
         return 0
      fi
   done
   return 1
}

echoerr() {
   echo "$@" 1>&2
}

url::open() {
   local -r cmd="$(platform::existing_command "${BROWSER:-}" xdg-open open google-chrome firefox)"
   "$cmd" "$@"
}