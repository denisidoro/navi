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
