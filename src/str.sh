#!/usr/bin/env bash

ESCAPE_CHAR="\034"
ESCAPE_CHAR_2="\035"
ESCAPE_CHAR_3="\036"

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
   local -r separator="${2:-  +}"

   if [ -n "$n" ]; then
      awk -F "$separator" --"{print \$$n}"
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

str::remove_empty_lines() {
   sed '/^$/d'
}

str::last_line() {
   tail -n1
}

str::as_column() {
   local -r txt="$(cat)"
   local -r separator="$1"

   if command_exists column; then
      echo "$txt" | column -t -s "$separator"
   else
      echo "$txt" | awk -F "$separator" -vOFS='  ' 'NF > 0 { $1 = $1 } 1'
   fi
}

str::with_line_numbers() {
   awk '{printf("%d %s\n", NR,$0)}'
}