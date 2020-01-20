#!/usr/bin/env bash

SELECTION_ESCAPE_STR="   "

selection_str::cleanup() {
   sed -E "s/  +/${SELECTION_ESCAPE_STR}/g"
}

selection_str::without_ellipsis() {
   tr -d "…"
}

selection_str::comment() {
   echo "$*" | awk -F "${SELECTION_ESCAPE_STR}" '{print $1}' | selection_str::without_ellipsis
}

selection_str::snippet() {
   echo "$*" | awk -F "${SELECTION_ESCAPE_STR}" '{print $2}' | selection_str::without_ellipsis | cmd::escape
}

selection_str::tags() {
   echo "$*" | awk -F "${SELECTION_ESCAPE_STR}" '{print $3}' | selection_str::without_ellipsis
}

selection::resolve_ellipsis() {
   local -r str="$(selection_str::cleanup)"
   local -r cheats="$*"

   if echo "$str" | grep -q "…"; then
      local -r comment="$(selection_str::comment "$str")"
      local -r snippet="$(selection_str::snippet "$str")"
      local -r tags="$(selection_str::tags "$str")"
      local -r cheat="$(cheat::from_tags "$cheats" "$tags")"

      local -r tags2="$(echo "$cheat" | head -n1 | str::sub 2)"
      local -r comment2="$(echo "$cheat" | grep "$comment" | str::last_line | str::sub 2)"
      local -r snippet2="$(echo "$cheat" | grep "$comment2" -A 999| str::last_paragraph_line)"

      echo "${comment2}${SELECTION_ESCAPE_STR}${snippet2}${SELECTION_ESCAPE_STR}${tags2}"
   else
      echo "$str"
   fi
}

selection::dict() {
   local -r cheats="$1"
   local -r key="${2:-}"
   local -r str="$(selection::resolve_ellipsis "$cheats")"

   local -r comment="$(selection_str::comment "$str")"
   local -r snippet="$(selection_str::snippet "$str")"
   local -r tags="$(selection_str::tags "$str")"

   dict::new comment "$comment" snippet "$snippet" tags "$tags" key "$key" | sed "s/'''/'/g"
}

selection::comment() {
   local -r selection="$1"
   dict::get "$selection" comment
}

selection::snippet() {
   local -r selection="$1"
   dict::get "$selection" snippet
}

selection::tags() {
   local -r selection="$1"
   dict::get "$selection" tags
}

selection::key() {
   local -r selection="$1"
   dict::get "$selection" key
}
