#!/usr/bin/env bash

selection_str::cleanup() {
  sed -E "s/  +/   /g"
}

selection_str::comment() {
  echo "$*" | awk -F "   " '{print $1}' 
}

selection_str::snippet() {
  echo "$*" | awk -F "   " '{print $2}' 
}

selection_str::tags() {
  echo "$*" | awk -F "   " '{print $3}' | tr -d '[' | tr -d ']'
}

selection::resolve_ellipsis() {
   local -r str="$(selection_str::cleanup)"
   local -r cheats="$*"

  if echo "$str" | grep -q "…"; then
    local -r comment="$(selection_str::comment "$str")"
    local -r snippet="$(selection_str::snippet "$str")"
    local -r tags="$(selection_str::tags "$str")"
    local -r cheat="$(cheat::from_tags "$cheats" "$tags")"

    echo "$(echo "$cheat" | grep "$(echo "$comment" | tr -d "…")" | str::sub 2)  ${snippet}  [${tags}]"
  else
    echo "$str"
  fi 
}

selection::dict() {
   local -r cheats="$1"
   local -r str="$(selection::resolve_ellipsis "$cheats" | selection_str::cleanup)"

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