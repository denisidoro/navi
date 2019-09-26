#!/usr/bin/env bash

cheat::find() {
   for path in $(echo "$NAVI_PATH" | tr ':' '\n'); do
      find "$path" -iname '*.cheat'
   done
}

cheat::read_all() {
   for cheat in $(cheat::find); do
      echo
      cat "$cheat"
      echo
   done
}

cheat::memoized_read_all() {
   if [ -n "${NAVI_CACHE:-}" ]; then
      echo "$NAVI_CACHE"
      return
   fi
   local -r cheats="$(cheat::read_all)"
   echo "$cheats"
}

cheat::pretty() {
   awk 'function color(c,s) {
           printf("\033[%dm%s\033[0m",30+c,s)
        }

      /^%/ { tags=" ["substr($0, 3)"]"; next }
      /^#/ { print color(4, $0) color(60, tags); next }
      /^\$/ { next }
   NF { print color(7, $0) color(60, tags); next }'
}

cheat::from_selection() {
   local -r cheats="$1"
   local -r selection="$2"

   local -r tags="$(dict::get "$selection" tags)"

   echo "$cheats" \
      | grep "% ${tags}" -A99999 \
      || (echoerr "No valid cheatsheet!"; exit 67)
}
