#!/usr/bin/env bash

# for an explanation behind this namespace, please check
# https://medium.com/@den.isidoro/dictionaries-in-shell-scripts-61d34e1c91c6

# LIMITATIONS:
# values with non-trivial whitespaces (newlines, subsequent spaces, etc)
# aren't handled very well

DICT_DELIMITER='\f'

dict::_post() {
   sed -E 's/; /\\n/g' | awk 'NF > 0' | dict::_unescape_value | sort
}

dict::new() {
   if [ $# = 0 ]; then
      echo ""
   else
      echo "" | dict::assoc "$@"
   fi
}

dict::dissoc() {
   local -r key="$1"

   grep -Ev "^${key}[^:]*:" | dict::_post
}

dict::_escape_value() {
   tr '\n' "$DICT_DELIMITER"
}

dict::_unescape_value() {
   tr "$DICT_DELIMITER" '\n'
}

dict::assoc() {
   local -r key="${1:-}"
   local -r value="$(echo "${2:-}" | dict::_escape_value)"
   local -r input="$(cat)"

   if [ -z $key ]; then
      printf "$input" | dict::_post
      return
   fi

   if [ -n "$input" ]; then
      local -r base="$(printf "$input" | dict::dissoc "$key"); "
   else
      local -r base=""
   fi

   shift 2
   printf "${base}${key}: ${value}" | dict::_post | dict::assoc "$@" | dict::_post
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
      echo "$result" | dict::_unescape_value
   else
      echo "$result" | sed -E "s/${prefix}//" | dict::_unescape_value | sed -E 's/^[[:space:]]*//'
   fi
}

dict::keys() {
   grep -Eo '^[^:]+: ' | sed 's/: //g'
}

dict::values() {
   awk -F':' '{$1=""; print $0}' | cut -c3-
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
   local -r updated_value="$(eval "$fn" "$value")"

   echo "$input" | dict::assoc "$key" "$updated_value"
}