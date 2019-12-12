#!/usr/bin/env bash

cheat::find() {
   for path in $(echo "$NAVI_PATH" | tr ':' '\n'); do
      find -L "${path/#\~/$HOME}" -iname '*.cheat'
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
   local -r print="$(dict::get "$OPTIONS" print)"

   local -r comment_width="$(style::comment_width)"
   local -r snippet_width="$(style::snippet_width)"
   local -r tag_width="$(style::tag_width)"

   local -r comment_color="$(style::comment_color)"
   local -r snippet_color="$(style::snippet_color)"
   local -r tag_color="$(style::tag_color)"

   local -r columns="$(ui::width || echo 0)"

   awk \
      -v COMMENT_COLOR=$comment_color \
      -v SNIPPET_COLOR=$snippet_color \
      -v TAG_COLOR=$tag_color \
      -v COMMENT_MAX=$((columns * comment_width / 100)) \
      -v SNIPPET_MAX=$((columns * snippet_width / 100)) \
      -v TAG_MAX=$((columns * tag_width / 100)) \
      -v SEP="$ESCAPE_CHAR_3" \
      'function color(c,s,max) {
           if (max > 0 && length(s) > max) {
              s=substr(s, 0, max)
              s=s"…"
           }
           printf("\033[%dm%s", c, s)
        }

      /^%/ { tags=substr($0, 3); next }
      /^#/ { comment=substr($0, 3); next }
      /^\$/ { next }
   BEGIN { ORS="" }
   NF {
    print color(COMMENT_COLOR, comment, COMMENT_MAX)
    print color(0, SEP, 0)
    print color(SNIPPET_COLOR, $0, SNIPPET_MAX)
    print color(0, SEP, 0)
    print color(TAG_COLOR, tags, TAG_MAX);
    print color(0, SEP, 0)
    print color(DEFAULT, "\033", 0);
    print "\n"
    next
   }'
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
