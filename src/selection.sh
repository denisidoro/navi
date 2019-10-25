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
   echo "$*" | awk -F "${SELECTION_ESCAPE_STR}" '{print $2}' | selection_str::without_ellipsis
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
      echo "$(echo "$cheat" | grep "$comment" | str::sub 2)${SELECTION_ESCAPE_STR}${snippet}${SELECTION_ESCAPE_STR}${tags}"
   else
      echo "$str"
   fi
}

selection::dict() {
   local -r cheats="$1"
   local -r str="$(selection::resolve_ellipsis "$cheats")"

   local -r comment="$(selection_str::comment "$str")"
   local -r snippet="$(selection_str::snippet "$str")"
   local -r tags="$(selection_str::tags "$str")"

   dict::new comment "$comment" snippet "$snippet" tags "$tags" | sed "s/'''/'/g"
}

selection::comment() {
   local -r selection="$1"
   dict::get "$selection" comment
}

selection::snippet() {
   local -r selection="$1"
   dict::get "$selection" snippet
}