#!/usr/bin/env bash

if ${NAVI_FORCE_GNU:-false} && [ -n "${DOTFILES:-}" ]; then
   source "${DOTFILES}/scripts/core/main.sh"
fi

source "${SCRIPT_DIR}/src/arg.sh"
source "${SCRIPT_DIR}/src/cheat.sh"
source "${SCRIPT_DIR}/src/coll.sh"
source "${SCRIPT_DIR}/src/dict.sh"
source "${SCRIPT_DIR}/src/health.sh"
source "${SCRIPT_DIR}/src/misc.sh"
source "${SCRIPT_DIR}/src/opts.sh"
source "${SCRIPT_DIR}/src/search.sh"
source "${SCRIPT_DIR}/src/selection.sh"
source "${SCRIPT_DIR}/src/str.sh"
source "${SCRIPT_DIR}/src/ui.sh"

handler::main() {
   local -r cheats="$(cheat::memoized_read_all)"
   cheat::export_cache "$cheats"
   local -r selection="$(ui::select "$cheats")"
   local -r cheat="$(cheat::from_selection "$cheats" "$selection")"

   [ -z "$cheat" ] && exit 67

   local -r interpolation="$(dict::get "$OPTIONS" interpolation)"
   local cmd="$(selection::cmd "$selection" "$cheat")"
   local arg value

   local -r args="$(dict::get "$OPTIONS" args)"

   local i=0
   while $interpolation; do
      arg="$(echo "$cmd" | arg::next || echo "")"
      if [ -z "$arg" ]; then
         break
      fi

      escaped_arg="$(echo "$arg" | tr '-' '_' | tr ' ' '_')"

      cmd="$(echo "$cmd" | sed "s|<${arg}>|<${escaped_arg}>|g")"
      arg="$escaped_arg"

      value="$(echo "$args" | coll::get $i)"
      [ -z "$value" ] && value="$(arg::pick "$arg" "$cheat")"

      if [ -z "$value" ]; then
         echoerr "Unable to fetch suggestions for '$arg'!"
         exit 1
      fi

      eval "local $arg"='$value'
      cmd="$(echo "$cmd" | arg::interpolate "$arg" "$value")"

      i=$((i+1))
   done

   local -r unresolved_arg="$(echo "$cmd" | arg::next || echo "")"

   local -r print="$(dict::get "$OPTIONS" print)"
   if $print || [ -n "$unresolved_arg" ]; then
      echo "$cmd"
   else
      eval "$cmd"
   fi
}

handler::preview() {
   local -r query="$1"
   local -r selection="$(echo "$query" | selection::dict)"
   local -r cheats="$(cheat::memoized_read_all)"
   local -r cheat="$(cheat::from_selection "$cheats" "$selection")"
   [ -n "$cheat" ] && selection::cmd_or_comment "$selection" "$cheat"
}

handler::help() {
   echo "$TEXT"
}

handler::version() {
   local -r full="${1:-false}"

   echo "${VERSION:-unknown}"

   if $full; then
      source "${SCRIPT_DIR}/src/version.sh"
      version::code 2>/dev/null \
         || echoerr "unknown code"
   fi
}

handler::script() {
   "${SCRIPT_DIR}/scripts/${SCRIPT_ARGS[@]}"
}

handler::home() {
   echo "${SCRIPT_DIR}"
}

handler::widget() {
   local widget
   local -r print="$(dict::get "$OPTIONS" print)"

   case "$SH" in
      zsh) widget="${SCRIPT_DIR}/navi.plugin.zsh" ;;
      bash) widget="${SCRIPT_DIR}/navi.plugin.bash" ;;
      *) echoerr "Invalid shell: $SH"; exit 1 ;;
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
