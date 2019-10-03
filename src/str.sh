#!/usr/bin/env bash

str::length() {
   awk '{print length}'
}

str::sub() {
   local -r start="${1:-0}"
   local -r finish="${2:-99999}"

   cut -c "$((start + 1))-$((finish - 1))"
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

str::reverse_lines() {
   if command_exists tac; then
      tac
   elif command_exists perl; then
      perl -e 'print reverse <>'
   else
      awk '{a[i++]=$0} END {for (j=i-1; j>=0;) print a[j--] }'
   fi
}

str::not_empty() {
   local -r input="$(cat)"

   if [ -n $input ]; then
      echo "$input"
   else
      return 1
   fi
}