#!/usr/bin/env bash

coll::new() {
   for x in "$@"; do
      echo "$x"
   done
}

coll::first() {
   head -n1
}

coll::rest() {
   tail -n +2
}

coll::map() {
   local -r fn="$1"

   for x in $(cat); do
      "$fn" "$x"
   done
}

coll::filter() {
   local -r pred="$1"

   for x in $(cat); do
      "$pred" "$x" && echo "$x" || true
   done
}

coll::remove() {
   local -r pred="$1"

   for x in $(cat); do
     "$pred" "$x" || echo "$x"
   done
}

coll::add() {
   cat
   for x in "$@"; do
      echo "$x"
   done
}

coll::reverse() {
   tac
}

coll::set() {
   sort -u
}

# TODO: implement tailrec
coll::reduce() {
   local -r fn="$1"
   local state="$2"

   local -r coll="$(cat)"
   local -r x="$(echo "$coll" | coll::first)"

   if [ -z "$x" ]; then
      echo "$state"
   else
      local -r new_state="$("$fn" "$state" "$x")"
      echo "$coll" | coll::rest | coll::reduce "$fn" "$new_state"
   fi
}