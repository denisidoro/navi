#!/usr/bin/env bash
set -euo pipefail

opts::extract_help() {
   local -r file="$1"
   grep "^##?" "$file" | cut -c 5-
}

opts::eval() {
   local wait_for=""
   local entry_point="main"
   local print=false
   local interpolation=true
   local preview=true
   local path="${NAVI_PATH:-${NAVI_DIR:-${SCRIPT_DIR}/cheats}}"

   for arg in "$@"; do
      case $wait_for in
         path) path="$arg"; wait_for="" ;;
         preview) query="$(arg::deserialize "$arg")"; wait_for="" ;;
         search) query="$arg"; wait_for=""; path="${path}:$(search::full_path "$query")"; ;;
         query) query="$arg"; wait_for="" ;;
      esac

      case $arg in
         --print) print=true ;;
         --no-interpolation) interpolation=false ;;
         --version) dict::new entry_point "text" text "${VERSION:-unknown}" && exit 0 ;;
         help|--help) dict::new entry_point "text" text "$(opts::extract_help "$0")" && exit 0 ;;
         --command-for) wait_for="command-for" ;;
         --no-preview) preview=false ;;
         --path) wait_for="path" ;;
         search) entry_point="search"; wait_for="search" ;;
         preview) entry_point="preview"; wait_for="preview" ;;
         q|query) wait_for="query" ;;
      esac
   done

   dict::new entry_point "$entry_point" print "$print" interpolation "$interpolation" preview "$preview" query "${query:-}" path "$path"
}
