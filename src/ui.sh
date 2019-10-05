#!/usr/bin/env bash

ui::pick() {
   local -r autoselect="$(dict::get "$OPTIONS" autoselect)"

   declare -a args
   args+=("--height")
   args+=("100%")
   if ${autoselect:-false}; then
      args+=("--select-1")
   fi

   local -r fzf_cmd="$([ $NAVI_ENV == "test" ] && echo "fzf_mock" || echo "fzf")"
   "$fzf_cmd" "${args[@]:-}" --inline-info "$@"
}

# TODO: separation of concerns
ui::select() {
   local -r cheats="$1"

   local -r script_path="${SCRIPT_DIR}/navi"
   local -r preview_cmd="echo \'{}\' | tr \"'\" '${ARG_DELIMITER_2}' | tr ' ' '${ARG_DELIMITER}' | tr '\"' '${ARG_DELIMITER_3}' | xargs -I% \"${script_path}\" preview %"

   local -r query="$(dict::get "$OPTIONS" query)"
   local -r entry_point="$(dict::get "$OPTIONS" entry_point)"
   local -r preview="$(dict::get "$OPTIONS" preview)"
   local -r best="$(dict::get "$OPTIONS" best)"

   local args=()
   args+=("-i")
   args+=("--ansi")
   if $preview; then
      args+=("--preview"); args+=("$preview_cmd")
      args+=("--preview-window"); args+=("up:1")
   fi
   if [[ -n "$query" ]] && $best; then
      args+=("--filter"); args+=("${query} ")
   elif  [[ -n "$query" ]] && ! $best; then
      args+=("--query"); args+=("${query} ")
   fi
   if [ "$entry_point" = "search" ]; then
      args+=("--header"); args+=("Displaying online results. Please refer to 'navi --help' for details")
   fi

   ui::_select_post() {
      if $best; then
         head -n1
      else
         cat
      fi
   }

   echo "$cheats" \
      | cheat::pretty \
      | ui::pick "${args[@]}" \
      | ui::_select_post \
      | selection::dict
}

ui::clear_previous_line() {
   tput cuu1 2>/dev/null && tput el || true
}
