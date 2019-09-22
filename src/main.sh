#!/usr/bin/env bash

source "${SCRIPT_DIR}/src/arg.sh"
source "${SCRIPT_DIR}/src/cheat.sh"
source "${SCRIPT_DIR}/src/health.sh"
source "${SCRIPT_DIR}/src/misc.sh"
source "${SCRIPT_DIR}/src/opts.sh"
source "${SCRIPT_DIR}/src/search.sh"
source "${SCRIPT_DIR}/src/selection.sh"
source "${SCRIPT_DIR}/src/str.sh"
source "${SCRIPT_DIR}/src/ui.sh"

handler::main() {
   local readonly cheats="$(cheat::find)"
   local readonly selection="$(ui::select "$cheats")"
   local readonly cheat="$(cheat::from_selection "$cheats" "$selection")"
   [ -z "$cheat" ] && exit 67
   local cmd="$(selection::command "$selection" "$cheat")"
   local arg value

   while $interpolation; do
      arg="$(echo "$cmd" | arg::next || echo "")"
      if [ -z "$arg" ]; then
         break
      fi

      value="$(arg::pick "$arg" "$cheat" || echo "")"
      if [ -z "$value" ]; then
         echo "$cmd"
         exit 0
      fi

      eval "local $arg"='$value'
      cmd="$(echo "$cmd" | arg::interpolate "$arg" "$value")"
   done

   local readonly unresolved_arg="$(echo "$cmd" | arg::next || echo "")"

   if $print || [ -n "$unresolved_arg" ]; then
      echo "$cmd"
   else
      eval "$cmd"
   fi
}

handler::preview() {
   local readonly selection="$(echo "$query" | selection::standardize)"
   local readonly cheats="$(cheat::find)"
   local readonly cheat="$(cheat::from_selection "$cheats" "$selection")"
   [ -n "$cheat" ] && selection::command "$selection" "$cheat"
}

main() {
   case ${entry_point:-} in
      preview) 
         handler::preview "$@"  \
            || echo "Unable to find command for '${query:-}'" 
            ;;
      search) 
         health::fzf 
         search::save "$query" || true
         handler::main "$@"
         ;;
      *) 
         health::fzf 
         handler::main "$@" 
         ;;
   esac
}