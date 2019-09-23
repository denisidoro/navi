#!/usr/bin/env bash

cheat::find() {
   for path in $(echo "$NAVI_PATH" | tr ':' '\n'); do
      find "$path" -iname '*.cheat'
   done
}

cheat::read_many() {
   for cheat in $(cat); do
      awk '
        function color(c,s) {
           printf("\033[%dm%s\033[0m",30+c,s)
        }

        /^%/ { tags=" ["substr($0, 3)"]"; next }
        /^#/ { print color(4, $0) color(60, tags); next }
        /^\$/ { next }
      NF { print color(7, $0) color(60, tags); next }' "$cheat"
   done
}

cheat::from_selection() {
   local -r cheats="$1"
   local -r selection="$2"

   local -r tags="$(echo "$selection" | selection::tags)"

   for cheat in $cheats; do
      if grep -q "% $tags" "$cheat"; then
         echo "$cheat"
         exit 0
      fi
   done

   echoerr "No valid cheatsheet!"
   exit 67
}
