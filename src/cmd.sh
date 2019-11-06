#!/usr/bin/env bash

cmd::escape() {
   tr '\\' "$ESCAPE_CHAR_3"
}

cmd::unescape() {
   tr "$ESCAPE_CHAR_3" '\\'
}

cmd::loop() {
   local -r cmd="$1"
   local -r cheat="$2"

   local arg escaped_arg value escaped_cmd

   arg="$(echo "$cmd" | arg::next)"
   if [ -z "$arg" ]; then
      dict::new cmd "$cmd"
      return
   fi

   escaped_arg="$(arg::escape "$arg")"

   escaped_cmd="$(echo "$cmd" | sed "s|<${arg}>|<${escaped_arg}>|g")"
   arg="$escaped_arg"

   local -r values="$(dict::get "$OPTIONS" values)"
   value="$(echo "$values" | coll::get $i)"
   [ -z "$value" ] && value="$(arg::pick "$arg" "$cheat")"

   dict::new \
      cmd "${escaped_cmd:-}" \
      value "$value" \
      arg "$arg"
}

cmd::finish() {
   local -r cmd="$(echo "$1" | cmd::unescape)"
   local -r selection="${2:-}"

   local -r key="$(selection::key "$selection")"
   local -r unresolved_arg="$(echo "$cmd" | arg::next)"

   local -r print="$(dict::get "$OPTIONS" print)"
   if [[ "$key" = "ctrl-y" ]]; then
      clip::set "$cmd"
   elif $print || [ -n "$unresolved_arg" ]; then
      echo "$cmd"
   else
      eval "$cmd"
   fi
}