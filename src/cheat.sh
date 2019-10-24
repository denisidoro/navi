#!/usr/bin/env bash

cheat::find() {
   for path in $(echo "$NAVI_PATH" | tr ':' '\n'); do
      find -L "$path" -iname '*.cheat'
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
  local -r columns="$(tput cols)"
   awk \
     -v COMMENT_MAX=$((columns * 48 / 100)) \
     -v SNIPPET_MAX=$((columns * 24 / 100)) \
     -v SEP="$ESCAPE_CHAR_3" \
     'function color(c,s,max) {
           if (max > 0 && length(s) > max) {
              s2=substr(s, 0, max)
              s2=s2"…"
           } else {
              s2=s
           }
           printf("\033[%dm%s", 30+c, s2)
        }

      /^%/ { tags="["substr($0, 3)"]"; next }
      /^#/ { comment=substr($0, 3); next }
      /^\$/ { next }
   NF { print color(4, comment, COMMENT_MAX) color(-30, SEP, 0) color(7, $0, SNIPPET_MAX) color(-30, SEP, 0) color(60, tags, 0); next }'
}

cheat::until_percentage() {
   awk 'BEGIN { count=0; }

      /^%/ { if (count >= 1) exit;
             else { count++; print $0; next; } }
   { print $0 }'
}

cheat::from_tags() {
   local -r cheats="$1"
   local -r tags="$2"

   echo "$cheats" \
      | grep "% ${tags}" -A99999 \
      | cheat::until_percentage
}

cheat::from_selection() {
   local -r cheats="$1"
   local -r selection="$2"

   local -r tags="$(dict::get "$selection" tags)"

   cheat::from_tags "$cheats" "$tags"
}
