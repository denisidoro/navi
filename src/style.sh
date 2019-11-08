#!/usr/bin/env bash

# TODO: move this elsewhere
style::get_index() {
   local -r txt="$1"
   local -r ref="$2"

   local -r i="$(echo "$txt" | grep "${ref}\$" | awk '{print $1}')"
   echo $((i - 1))
}

# TODO: move this elsewhere
style::with_nth() {
   grep -Eo 'with\-nth +([^ ]+)' | awk '{print $NF}'
}

style::width() {
   local -r col="$1"
   local -r print="$(dict::get "$OPTIONS" print)"
   local -r widths="$(dict::get "$OPTIONS" col-widths | tr ',' $'\n')"
   local -r numbered_with_nth="$(dict::get "$OPTIONS" fzf-overrides | style::with_nth | tr ',' $'\n' | str::with_line_numbers)"

   if [ -n "$numbered_with_nth" ]; then
      local -r index="$(style::get_index "$numbered_with_nth" $col 2>/dev/null)"
      echo "$widths" | coll::get $index 2>/dev/null || echo 0
   else
      echo 0
   fi
}

style::color() {
   local -r col="$1"
   local -r print="$(dict::get "$OPTIONS" print)"
   local -r colors="$(dict::get "$OPTIONS" col-colors | tr ',' $'\n')"
   local -r numbered_with_nth="$(dict::get "$OPTIONS" fzf-overrides | style::with_nth | tr ',' $'\n' | str::with_line_numbers)"

   if [ -n "$numbered_with_nth" ]; then
      local -r index="$(style::get_index "$numbered_with_nth" $col 2>/dev/null)"
      echo "$colors" | coll::get $index 2>/dev/null || echo 0
   else
      echo 30
   fi
}

style::comment_width() { style::width 1 "$@"; }
style::snippet_width() { style::width 2 "$@"; }
style::tag_width() { style::width 3 "$@"; }

style::comment_color() { style::color 1 "$@"; }
style::snippet_color() { style::color 2 "$@"; }
style::tag_color() { style::color 3 "$@"; }

