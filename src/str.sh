#!/usr/bin/env bash

str::length() {
   awk '{print length}'
}

str::sub() {
   local -r text="$(cat)"

   local -r start="${1:-0}"
   local finish="${2:-99999}"

   if [[ $finish < 0 ]]; then
      local -r length=$(echo "$text" | str::length)
      finish=$((length-finish))
   fi

   echo "$text" | cut -c "$((start + 1))-$((finish - 1))"
}

str::column() {
   local -r n="${1:-}"

   if [ -n "$n" ]; then
      awk "{print \$$n}"
   else
      cat
   fi
}

str::last_paragraph_line() {
   awk '(!NF) { exit } { print $0 }' \
      | tail -n1
}

str::first_word() {
   awk '{print $1}'
}

str::index_last_occurrence() {
   local -r char="$1"
   
   awk 'BEGIN{FS=""}{ for(i=1;i<=NF;i++){ if($i=="'"$char"'"){ p=i } }}END{  print p }'
}
