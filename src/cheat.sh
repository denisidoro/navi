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
     -v COMMENT_MAX=$((columns * 55 / 100)) \
     -v SNIPPET_MAX=0 \
     -v SEP="$ESCAPE_CHAR_3" \
     'function color(c,s,max) {
           if (max > 0 && length(s) > max) {
              s=substr(s, 0, max)
              s=s"…"
           }
           printf("\033[%dm%s", c, s)
        }

      /^%/ { tags="["substr($0, 3)"]"; next }
      /^#/ { comment=substr($0, 3); next }
      /^\$/ { next }
   NF { print color(34, comment, COMMENT_MAX) color(0, SEP, 0) color(37, $0, SNIPPET_MAX) color(0, SEP, 0) color(90, tags, 0); next }'
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
