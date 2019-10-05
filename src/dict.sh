#!/usr/bin/env bash

# for an explanation behind this namespace, please check
# https://medium.com/@den.isidoro/dictionaries-in-shell-scripts-61d34e1c91c6

# LIMITATIONS:
# values with non-trivial whitespaces (newlines, subsequent spaces, etc)
# aren't handled very well

dict::new() {
   if [ $# = 0 ]; then
      echo ""
   else
      echo "" | dict::assoc "$@" | str::remove_empty_lines
   fi
}

dict::dissoc() {
   local -r key="$1"

   grep -Ev "^[\s]*${key}[^:]*:"
}

dict::escape_value() {
   tr '\n' "$ESCAPE_CHAR" | sed 's/\\n/'$(printf "$ESCAPE_CHAR")'/g'
}

str::without_trailing_newline() {
   printf "%s" "$(cat)"
   echo
}

dict::unescape_value() {
   tr "$ESCAPE_CHAR" '\n' | str::without_trailing_newline
}

dict::assoc() {
   local -r key="${1:-}"
   local -r input="$(cat)"

   if [ -z $key ]; then
      printf "$(echo "$input" | tr '%' "$ESCAPE_CHAR_2")" | tr "$ESCAPE_CHAR_2" '%'
      return
   fi

   local -r value="$(echo "${2:-}" | dict::escape_value)"

   shift 2
   echo "$(echo "$input" | dict::dissoc "$key")${key}: ${value}\n" | dict::assoc "$@"
}

dict::get() {
   if [ $# = 1 ]; then
      local -r input="$(cat)"
      local -r key="$1"
   else
      local -r input="$1"
      local -r key="$2"
   fi

   local -r prefix="${key}[^:]*: "
   local -r result="$(echo "$input" | grep -E "^${prefix}")"
   local -r matches="$(echo "$result" | wc -l || echo 0)"

   if [ $matches -gt 1 ]; then
      echo "$result" | dict::unescape_value
   else
      echo "$result" | sed -E "s/${prefix}//" | dict::unescape_value
   fi
}

dict::keys() {
   grep -Eo '^[^:]+: ' \
      | sed 's/: //g'
}

dict::values() {
   awk -F':' '{$1=""; print $0}' \
      | cut -c3-
}

dict::zipmap() {
   IFS='\n'

   local -r keys_str="$1"
   local -r values_str="$2"

   keys=()
   values=()
   for key in $keys_str; do
      keys+=("$key")
   done
   for value in $values_str; do
      values+=("$value")
   done

   for ((i=0; i<${#keys[@]}; ++i)); do
      if [ -n "${keys[i]}" ]; then
         echo "${keys[i]}: ${values[i]}"
      fi
   done
}

dict::update() {
   local -r key="$1"
   local -r fn="$2"
   local -r input="$(cat)"

   local -r value="$(echo "$input" | dict::get "$key")"
   local -r updated_value="$("$fn" "$value")"

   echo "$input" | dict::assoc "$key" "$updated_value"
}