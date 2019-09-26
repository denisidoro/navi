#!/usr/bin/env bash

if ${NAVI_FORCE_GNU:-false}; then
   source "${DOTFILES}/scripts/core/main.sh" 2>/dev/null || true
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
   local -r cheats="$(cheat::find)"
   local -r selection="$(ui::select "$cheats")"
   local -r cheat="$(cheat::from_selection "$cheats" "$selection")"

   [ -z "$cheat" ] && exit 67

   local -r interpolation="$(dict::get "$OPTIONS" interpolation)"
   local cmd="$(selection::cmd "$selection" "$cheat")"
   local arg value

   while $interpolation; do
      arg="$(echo "$cmd" | arg::next || echo "")"
      if [ -z "$arg" ]; then
         break
      fi

      value="$(arg::pick "$arg" "$cheat" || echo "")"
      if [ -z "$value" ]; then
         echoerr "Unable to fetch suggestions for '$arg'!"
         exit 0
      fi

      eval "local $arg"='$value'
      cmd="$(echo "$cmd" | arg::interpolate "$arg" "$value")"
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
   local -r cheats="$(cheat::find)"
   local -r cheat="$(cheat::from_selection "$cheats" "$selection")"
   [ -n "$cheat" ] && selection::cmd "$selection" "$cheat"
}

handler::help() {
   echo "$TEXT"
}

handler::version() {
   local -r full="${1:-false}"

   echo "${VERSION:-unknown}"

   if $full; then
      source "${SCRIPT_DIR}/src/version.sh"
      version::code 2>/dev/null || echo "unknown code"
   fi
}

handler::script() {
   "${SCRIPT_DIR}/scripts/${SCRIPT_ARGS[@]}"
}

handler::home() {
   echo "${SCRIPT_DIR}"
}

main() {
   case "$(dict::get "$OPTIONS" entry_point)" in
      preview)
         local -r query="$(dict::get "$OPTIONS" query)"
         handler::preview "$query"  \
            || echo "Unable to find command for '$query'"
         echo "$NAVI_PATH"
         ;;
      search)
         health::fzf
         local -r query="$(dict::get "$OPTIONS" query)"
         search::save "$query" || true
         handler::main
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
      *)
         health::fzf
         handler::main
         ;;
   esac
}