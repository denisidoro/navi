#!/usr/bin/env bash

if ${NAVI_FORCE_GNU:-false} && [ -n "${DOTFILES:-}" ]; then
   source "${DOTFILES}/scripts/core/main.sh"
fi

source "${NAVI_HOME}/src/arg.sh"
source "${NAVI_HOME}/src/cheat.sh"
source "${NAVI_HOME}/src/cmd.sh"
source "${NAVI_HOME}/src/coll.sh"
source "${NAVI_HOME}/src/dict.sh"
source "${NAVI_HOME}/src/health.sh"
source "${NAVI_HOME}/src/misc.sh"
source "${NAVI_HOME}/src/opts.sh"
source "${NAVI_HOME}/src/search.sh"
source "${NAVI_HOME}/src/selection.sh"
source "${NAVI_HOME}/src/str.sh"
source "${NAVI_HOME}/src/ui.sh"

handler::main() {
   local -r cheats="$(cheat::memoized_read_all)"
   cheat::export_cache "$cheats"
   local -r selection="$(ui::select "$cheats")"
   local -r cheat="$(cheat::from_selection "$cheats" "$selection")"
   [ -z "$cheat" ] && die "No valid cheatsheet!"

   local -r interpolation="$(dict::get "$OPTIONS" interpolation)"

   local cmd="$(selection::cmd "$selection" "$cheat")"
   local result arg value

   local i=0
   while $interpolation; do
      result="$(cmd::loop "$cmd" "$cheat")"
      arg="$(dict::get "$result" arg)"
      value="$(dict::get "$result" value)"
      cmd="$(dict::get "$result" cmd)"

      [ -z "$arg" ] && break
      [ -z "$value" ] && die "Unable to fetch suggestions for '$arg'!"

      eval "local $arg"='$value'
      cmd="$(echo "$cmd" | arg::interpolate "$arg" "$value")"

      i=$((i+1))
   done

   cmd::finish "$cmd"
}

handler::preview() {
   local -r query="$1"
   local -r selection="$(echo "$query" | selection::dict)"
   local -r cheats="$(cheat::memoized_read_all)"
   local -r cheat="$(cheat::from_selection "$cheats" "$selection")"
   [ -n "$cheat" ] && selection::cmd_or_comment "$selection" "$cheat"
}

handler::help() {
   opts::extract_help "$0"
}

handler::version() {
   local -r full="${1:-false}"

   echo "${VERSION:-unknown}"

   if $full; then
      source "${NAVI_HOME}/src/version.sh"
      version::code 2>/dev/null \
         || die "unknown code"
   fi
}

handler::script() {
   "${NAVI_HOME}/scripts/"${SCRIPT_ARGS[@]}
}

handler::fn() {
   ${SCRIPT_ARGS[@]}
}

handler::home() {
   echo "${NAVI_HOME}"
}

handler::widget() {
   local widget
   local -r print="$(dict::get "$OPTIONS" print)"

   case "$SH" in
      zsh) widget="${NAVI_HOME}/navi.plugin.zsh" ;;
      bash) widget="${NAVI_HOME}/navi.plugin.bash" ;;
      fish) widget="${NAVI_HOME}/navi.plugin.fish" ;;
      *) die "Invalid shell: $SH" ;;
   esac

   $print \
      && cat "$widget" \
      || echo "$widget"
}

handler::search() {
   local -r query="$(dict::get "$OPTIONS" query)"
   search::save "$query" || true
   handler::main
}

main() {
   case "$(dict::get "$OPTIONS" entry_point)" in
      preview)
         local -r query="$(dict::get "$OPTIONS" query)"
         handler::preview "$query"  \
            || echoerr "Unable to find command for '$query'"
         ;;
      search)
         health::fzf
         handler::search
         ;;
      version)
         handler::version false
         ;;
      full-version)
         handler::version true
         ;;
      home)
         handler::home
         ;;
      script)
         handler::script
         ;;
      fn)
         handler::fn
         ;;
      help)
         handler::help
         ;;
      widget)
         handler::widget
         ;;
      *)
         health::fzf
         handler::main
         ;;
   esac
}
