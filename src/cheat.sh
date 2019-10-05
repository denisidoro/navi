#!/usr/bin/env bash

cheat::find() {
   for path in $(echo "$NAVI_PATH" | tr ':' '\n'); do
      find "$path" -iname '*.cheat'
   done
}

cheat::export_cache() {   
   if [ -z "${NAVI_CACHE:-}" ]; then
      export NAVI_CACHE="$*"
   fi
}

cheat::join_lines() {
   if command_exists perl; then
      perl -0pe 's/\\\n *//g'
   else
      tr '\n' "$ESCAPE_CHAR" \
         | sed -E 's/\\'$(printf "$ESCAPE_CHAR")' *//g' \
         | tr "$ESCAPE_CHAR" '\n'
   fi
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
   echo "$cheats" \
      | cheat::join_lines
}

cheat::prettify() {
   awk 'function color(c,s) {
           printf("\033[%dm%s\033[0m",30+c,s)
        }

      /^%/ { tags=" ["substr($0, 3)"]"; next }
      /^#/ { print color(4, $0) color(60, tags); next }
      /^\$/ { next }
   NF { print color(7, $0) color(60, tags); next }'
}

cheat::until_percentage() {
   awk 'BEGIN { count=0; }

      /^%/ { if (count >= 1) exit;
             else { count++; print $0; next; } }
   { print $0 }'
}

cheat::from_selection() {
   local -r cheats="$1"
   local -r selection="$2"

   local -r tags="$(dict::get "$selection" tags)"

   echo "$cheats" \
      | grep "% ${tags}" -A99999 \
      | cheat::until_percentage
}
