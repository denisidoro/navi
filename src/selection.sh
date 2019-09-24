#!/usr/bin/env bash

selection::dict() {
   local -r str="$(cat)"

   local -r index_last_bracket="$(echo "$str" | str::index_last_occurrence "[")"
   local -r index=$((index_last_bracket-1))

   local -r core="$(echo "$str" | str::sub 0 "$index")"
   local -r tags="$(echo "$str" | str::sub $((index+1)) -1)"

   dict::new core "$core" tags "$tags"
}

selection::core_is_comment() {
   grep -qE '^#'
}

selection::cmd() {
   local -r selection="$1"
   local -r cheat="$2"

   local -r core="$(echo "$selection" | dict::get core)"

   if echo "$core" | selection::core_is_comment; then
      grep "$core" "$cheat" -A999 \
         | str::last_paragraph_line
   else
      echo "$core"
   fi
}
