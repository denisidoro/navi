#!/usr/bin/env bash

ui::fzf() {
   local -r autoselect="$(dict::get "$OPTIONS" autoselect)"
   local -r with_nth="$(dict::get "$OPTIONS" with-nth)"
   local -r nth="$(dict::get "$OPTIONS" nth)"

   local args
   args+=("--height")
   args+=("100%")
   if ${autoselect:-false}; then
      args+=("--select-1")
   fi
   args+=("--bind"); args+=("ctrl-j:down,ctrl-k:up")

   local -r fzf_cmd="$([ $NAVI_ENV == "test" ] && echo "fzf_mock" || echo "fzf")"
   "$fzf_cmd" ${args[@]:-} --inline-info "$@"
}

ui::select() {
   local -r cheats="$1"

   [[ "${SHELL:-}" =~ 'fish' ]] || local -r sub='$'

   local -r script_path="${NAVI_HOME}/navi"
   local -r preview_cmd="\"${script_path}\" preview ${sub:-}(echo \'{}\' | $(arg::serialize_code))"

   local -r query="$(dict::get "$OPTIONS" query)"
   local -r entry_point="$(dict::get "$OPTIONS" entry_point)"
   local -r preview="$(dict::get "$OPTIONS" preview)"
   local -r best="$(dict::get "$OPTIONS" best)"
   local -r fzf_overrides="$(dict::get "$OPTIONS" fzf-overrides)"

   local args=()
   args+=("-i")
   args+=("--ansi")
   if $preview; then
      args+=("--preview"); args+=("$preview_cmd")
      args+=("--preview-window"); args+=("up:2")
   fi
   if [[ -n "$query" ]] && $best; then
      args+=("--filter"); args+=("${query} ")
   elif  [[ -n "$query" ]] && ! $best; then
      args+=("--query"); args+=("${query} ")
   fi
   if [ "$entry_point" = "search" ]; then
      args+=("--header"); args+=("Displaying online results. Please refer to 'navi --help' for details")
   fi
   args+=("--delimiter"); args+=('\s\s+')
   args+=("--expect"); args+=("ctrl-y")

   export FZF_DEFAULT_OPTS="${FZF_DEFAULT_OPTS} ${fzf_overrides}"

   local -r result="$(echo "$cheats" \
      | cheat::prettify \
      | str::as_column $(printf "$ESCAPE_CHAR_3") \
      | ui::fzf "${args[@]}")"

   local -r key="$(echo "$result" | head -n1)"

   echo "$result" \
      | ($best && head -n1 || tail -n +2) \
      | selection::dict "$cheats" "$key"
}

ui::clear_previous_line() {
   tput cuu1 2>/dev/null && tput el || true
}

ui::width() {
   shopt -s checkwinsize; (:;:) 2> /dev/null || true
   if command_exists tput; then
      tput cols
   else
      echo 130
   fi
}

ui::remove_dep_order() {
   sed -E 's/^: [^;]+; //'
}

ui::print_preview() {
   local -r selection="$1"

   local -r comment="$(selection::comment "$selection" | cmd::unescape)"
   local -r snippet="$(selection::snippet "$selection" | cmd::unescape)"
   local -r tags="$(selection::tags "$selection" | cmd::unescape)"

   local -r comment_color="$(style::comment_color)"
   local -r snippet_color="$(style::snippet_color)"
   local -r tag_color="$(style::tag_color)"

   printf "\033[${comment_color}m# "; echo -n "$comment"
   printf " \033[${tag_color}m["; echo -n "$tags"; echo "]"
   printf "\033[${snippet_color}m"
   echo "$snippet" | ui::remove_dep_order
}
