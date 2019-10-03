#!/usr/bin/env bash

selection::dict() {
   local -r str="$(cat)"

   local -r tags="$(echo "$str" | awk -F'[' '{print $NF}' | tr -d ']')"
   local -r core="$(echo "$str" | sed -e "s/ \[${tags}\]$//")"

   dict::new core "$core" tags "$tags" | sed "s/'''/'/g"
}

selection::core_is_comment() {
   grep -qE '^#'
}

selection::cmd_or_comment() {
   local -r selection="$1"
   local -r cheat="$2"
   local -r always_cmd="${3:-false}"

   local -r core="$(echo "$selection" | dict::get core)"

   if echo "$core" | selection::core_is_comment; then
      echo "$cheat" \
         | grep "$core" -A999 \
         | str::last_paragraph_line
   elif $always_cmd; then
      echo "$core"
   else
      echo "$cheat" \
         | grep "^${core}$" -B999 \
         | str::reverse_lines \
         | str::last_paragraph_line
   fi
}

selection::cmd() {
   selection::cmd_or_comment "$@" true
}
