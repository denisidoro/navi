#!/usr/bin/env bash

check_all_vars() {
   local arg
   IFS=$'\n'
   for var in $(cat "$1" | grep -Eo "<[^>]*>"); do
      if ! echo "$var" | grep -qE "$ARG_REGEX"; then
         echoerr "$var isn't a valid variable name!"
         return 1
      fi
   done
}

for cheat in $(cheat::find); do
   test::run "All variables in $(basename $cheat) are valid" \
      'check_all_vars "$cheat"'
done
