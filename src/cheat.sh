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

# TODO: move this elsewhere
cheat::get_index() {
   local -r txt="$1"
   local -r ref="$2"

   local -r i="$(echo "$txt" | grep "${ref}\$" | awk '{print $1}')"
   echo $((i - 1))
}

# TODO: move this elsewhere
cheat::with_nth() {
   grep -Eo 'with\-nth +([^ ]+)' | awk '{print $NF}'
}

cheat::prettify() {
   local -r print="$(dict::get "$OPTIONS" print)"
   local -r widths="$(dict::get "$OPTIONS" col-widths | tr ',' $'\n')"
   local -r numbered_with_nth="$(dict::get "$OPTIONS" fzf-overrides | cheat::with_nth | tr ',' $'\n' | str::with_line_numbers)"

   if [ -n "$numbered_with_nth" ]; then
      local -r comment_index="$(cheat::get_index "$numbered_with_nth" 1 2>/dev/null)"
      local -r snippet_index="$(cheat::get_index "$numbered_with_nth" 2 2>/dev/null)"
      local -r tag_index="$(cheat::get_index "$numbered_with_nth" 3 2>/dev/null)"
      local -r comment_width="$(echo "$widths" | coll::get $comment_index 2>/dev/null || echo 0)"
      local -r snippet_width="$(echo "$widths" | coll::get $snippet_index 2>/dev/null || echo 0)"
      local -r tag_width="$(echo "$widths" | coll::get $tag_index 2>/dev/null || echo 0)"
      local -r columns="$(ui::width)"
   else
      local -r comment_width=0
      local -r snippet_width=0
      local -r tag_width=0
      local -r columns=0
   fi

   awk \
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
    print color(34, comment, COMMENT_MAX)
    print color(0, SEP, 0)
    print color(37, $0, SNIPPET_MAX)
    print color(0, SEP, 0)
    print color(90, tags, TAG_MAX);
    print color(0, SEP, 0)
    print color(90, "\033", 0);
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
