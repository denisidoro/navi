#!/usr/bin/env bash

ARG_REGEX_WITHOUT_BRACKETS="[a-zA-Z_]+([- ]?\w+)*"
ARG_REGEX="<${ARG_REGEX_WITHOUT_BRACKETS}>"
ARG_DELIMITER="\f"
ARG_DELIMITER_2="\v"
ARG_DELIMITER_3="\r"

arg::dict() {
   local -r input="$(cat | sed 's/\\n/\\f/g')"

   local -r fn="$(echo "$input" | awk -F'---' '{print $1}')"
   local -r opts="$(echo "$input" | awk -F'---' '{print $2}')"

   dict::new fn "$fn" opts "$opts"
}

arg::interpolate() {
   local -r arg="$1"
   local -r value="$2"

   local -r words="$(echo "$value" | wc -w | xargs)"

   if [[ $words > 1 ]]; then
      sed "s|<${arg}>|\"${value}\"|g"
   else
      sed "s|<${arg}>|${value}|g"
   fi
}

arg::next() {
   grep -Eo "$ARG_REGEX" \
      | head -n1 \
      | tr -d '<' \
      | tr -d '>'
}

arg::deserialize() {
   local arg="$1"

   arg="${arg:1:${#arg}-2}"
   echo "$arg" \
      | tr "${ARG_DELIMITER}" " " \
      | tr "${ARG_DELIMITER_2}" "'" \
      | tr "${ARG_DELIMITER_3}" '"'
}

# TODO: separation of concerns
arg::pick() {
   local -r arg="$1"
   local -r cheat="$2"

   local -r prefix="$ ${arg}:"
   local -r length="$(echo "$prefix" | str::length)"
   local -r arg_dict="$(echo "$cheat" | grep "$prefix" | str::sub $((length + 1)) | arg::dict)"

   local -r fn="$(dict::get "$arg_dict" fn | sed 's/\\f/\\n/g')"
   local -r args_str="$(dict::get "$arg_dict" opts)"
   local arg_name=""

   for arg_str in $args_str; do
      if [ -z $arg_name ]; then
         arg_name="$(echo "$arg_str" | str::sub 2)"
      else
         eval "local $arg_name"='$arg_str'
         arg_name=""
      fi
   done

   if [ -n "$fn" ]; then
      local suggestions="$(eval "$fn" 2>/dev/null)"
      if [ -n "$suggestions" ]; then
         echo "$suggestions" | ui::pick --prompt "$arg: " --header-lines "${headers:-0}" | str::column "${column:-}"
      fi
   elif ${NAVI_USE_FZF_ALL_INPUTS:-false}; then
      echo "" | ui::pick --prompt "$arg: " --print-query --no-select-1 --height 1
   else
      printf "\033[0;36m${arg}:\033[0;0m " > /dev/tty
      read -r value
      ui::clear_previous_line > /dev/tty
      printf "$value"
   fi
}
