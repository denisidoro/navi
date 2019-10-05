#!/usr/bin/env bash

command_exists() {
   local -r cmd="${1:-}"
   [ -n $cmd ] && type "$cmd" &> /dev/null
}

platform::existing_command() {
   local cmd
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
   "$cmd" "$@" & disown
}

tap() {
   local -r input="$(cat)"
   echoerr "$input"
   echo "$input"
}

die() {
   echoerr "$@"
   exit 42
}