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

coll::without_empty_line() {
   local -r input="$(cat)"
   local -r words="$(echo "$input" | wc -w | xargs)"
   if [[ $words > 0 ]]; then
      echo "$input"
   fi
}

coll::add() {
   cat | coll::without_empty_line
   for x in "$@"; do
      echo "$x"
   done
}

coll::reverse() {
   str::reverse_lines "$@"
}

coll::set() {
   sort -u
}

coll::get() {
   local n="$1"
   n=$((n+1))
   sed "${n}q;d"
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