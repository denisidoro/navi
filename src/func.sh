#!/usr/bin/env bash

func::list() {
   for x in "$@"; do
      echo "$x"
   done
}

func::map() {
   local -r fn="$1"
   shift

   for x in $(cat); do
      "$fn" "$x"
   done
}

func::reduce() {
   local -r fn="$1"
   local state="$2"

   local -r coll="$(cat)"
   local -r x="$(echo "$coll" | head -n1)"

   if [ -z "$x" ]; then
      echo "$state"
   else
      local -r new_state="$("$fn" "$state" "$x")"
      local -r new_coll="$(echo "$coll" | tail -n +2)"
      echo "$new_coll" | func::reduce "$fn" "$new_state"
   fi
}