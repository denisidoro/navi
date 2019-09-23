#!/usr/bin/env bash

selection::standardize() {
   local -r str="$(cat)"

   local -r tags="$(echo "$str" | awk -F'[' '{print $NF}' | tr -d ']')"
   local -r core="$(echo "$str" | sed -e "s/ \[${tags}\]$//")"

   dict::new core "$core" tags "$tags"
}

selection::core_is_comment() {
   grep -qE '^#'
}

selection::dict() {
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
