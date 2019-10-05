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

cheat::_join_multiline_using_sed() {
   tr '\n' '\f' \
      | sed -E 's/\\\f *//g' \
      | tr '\f' '\n'
}

cheat::_join_multiline() {
   if ${NAVI_USE_PERL:-false}; then
      perl -0pe 's/\\\n *//g' \
         || cheat::_join_multiline_using_sed
   else
      cheat::_join_multiline_using_sed
   fi
}

cheat::read_all() {
   for cheat in $(cheat::find); do
      echo
      cat "$cheat" | cheat::_join_multiline
      echo
   done
}

cheat::memoized_read_all() {
   if [ -n "${NAVI_CACHE:-}" ]; then
      echo "$NAVI_CACHE"
      return
   fi
   if command_exists perl; then
      export NAVI_USE_PERL=true
   else
      export NAVI_USE_PERL=false
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

cheat::_until_percentage() {
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
      | cheat::_until_percentage \
      || (echoerr "No valid cheatsheet!"; exit 67)
}
