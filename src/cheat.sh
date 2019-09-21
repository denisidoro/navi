#!/usr/bin/env bash

cheat::find() {
   find "${NAVI_DIR:-"${SCRIPT_DIR}/cheats"}" -iname '*.cheat'
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
         break
      fi
   done
}
