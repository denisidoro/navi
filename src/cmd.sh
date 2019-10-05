#!/usr/bin/env bash

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
   local -r cmd="$1"

   local -r unresolved_arg="$(echo "$cmd" | arg::next)"

   local -r print="$(dict::get "$OPTIONS" print)"
   if $print || [ -n "$unresolved_arg" ]; then
      echo "$cmd"
   else
      eval "$cmd"
   fi
}