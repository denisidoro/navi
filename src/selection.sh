#!/usr/bin/env bash

selection::dict() {
   local -r str="$(sed -E "s/  +/   /g")"

   local -r comment="$(echo "$str" | awk -F "   " '{print $1}')"
   local -r snippet="$(echo "$str" | awk -F "   " '{print $2}')"
   local -r tags="$(echo "$str" | awk -F "   " '{print $3}' | tr -d '[' | tr -d ']')"

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
