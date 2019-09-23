#!/usr/bin/env bash

ARG_REGEX="<[0-9a-zA-Z_]+>"

arg::dict() {
   local -r fn="$(awk -F'---' '{print $1}')"
   local -r opts="$(awk -F'---' '{print $2}')"

   dict::new fn "$fn" opts "$opts"
}

arg::interpolate() {
   local -r arg="$1"
   local -r value="$2"

   sed "s|<${arg}>|\"${value}\"|g"
}

arg::next() {
   grep -Eo "$ARG_REGEX" \
      | head -n1 \
      | tr -d '<' \
      | tr -d '>'
}

arg::pick() {
   local -r arg="$1"
   local -r cheat="$2"

   local -r prefix="$ ${arg}:"
   local -r length="$(echo "$prefix" | str::length)"
   local -r arg_dict="$(grep "$prefix" "$cheat" | str::sub $((length + 1)) | arg::dict)"

   local -r fn="$(echo "$arg_dict" | dict::get fn)"
   local -r args_str="$(echo "$arg_dict" | dic::get opts | tr ' ' '\n' || echo "")"
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
      eval "$fn" | ui::pick --prompt "$arg: " --header-lines "${headers:-0}" | str::column "${column:-}"
   else
      printf "\033[0;36m${arg}:\033[0;0m " > /dev/tty
      read value
      ui::clear_previous_line > /dev/tty
      printf "$value"
   fi
}
