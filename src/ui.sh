#!/usr/bin/env bash

ui::fzf() {
   local -r autoselect="$(dict::get "$OPTIONS" autoselect)"

   local args
   args+=("--height")
   args+=("100%")
   if ${autoselect:-false}; then
      args+=("--select-1")
   fi

   local -r fzf_cmd="$([ $NAVI_ENV == "test" ] && echo "fzf_mock" || echo "fzf")"
   "$fzf_cmd" ${args[@]:-} --inline-info "$@"
}

ui::select() {
   local -r cheats="$1"

   local -r script_path="${NAVI_HOME}/navi"
   local -r preview_cmd="echo \'{}\' | $(arg::serialize_code) | xargs -I% \"${script_path}\" preview %"

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

   echo "$cheats" \
      | cheat::prettify \
      | ui::fzf "${args[@]}" \
      | ($best && head -n1 || cat) \
      | selection::dict
}

ui::clear_previous_line() {
   tput cuu1 2>/dev/null && tput el || true
}
