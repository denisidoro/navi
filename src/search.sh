#!/usr/bin/env bash

search::cheat() {
   local readonly cmd="$(echo "$1" | str::first_word)"

   echo "% ${cmd}, cheatsh"
   echo
   curl -s "${CHTSH_URL:-http://cht.sh}/${cmd}?T"
}

search::filename() {
   local readonly cmd="$(echo "$1" | str::first_word)"

   echo "${cmd}_cheatsh" \
      | head -n1 \
      | awk '{print $NF}' \
      | xargs \
      | tr ' ' '_'
}

search::full_path() {
   local readonly cmd="$(echo "$1" | str::first_word)"

   echo "/tmp/navi/$(search::filename "$cmd").cheat"
}

search::save() {
   local readonly cmd="$(echo "$1" | str::first_word)"

   local readonly filepath="$(search::full_path "$cmd")"
   local readonly filedir="$(dirname "$filepath")"
   
   if [ -f "$filepath" ]; then
      return
   fi

   mkdir -p "$filedir" &> /dev/null || true
   search::cheat "$cmd" > "$filepath"
}