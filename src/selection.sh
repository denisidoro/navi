#!/usr/bin/env bash

selection::standardize() {
   local readonly str="$(cat)"

   local readonly tags="$(echo "$str" | awk -F'[' '{print $NF}' | tr -d ']')"
   local readonly core="$(echo "$str" | sed -e "s/ \[${tags}\]$//")"

   echo "${core}^${tags}"
}

selection::core() {
   cut -d'^' -f1
}

selection::tags() {
   cut -d'^' -f2
}

selection::core_is_comment() {
   grep -qE '^#'
}

selection::command() {
   local readonly selection="$1"
   local readonly cheat="$2"

   local readonly core="$(echo $selection | selection::core)"

   if echo "$core" | selection::core_is_comment; then
      grep "$core" "$cheat" -A999 \
         | str::last_paragraph_line
   else
      echo "$core"
   fi
}
