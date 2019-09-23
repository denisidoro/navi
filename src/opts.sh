#!/usr/bin/env bash
set -euo pipefail

opts::extract_help() {
   local -r file="$1"
   grep "^##?" "$file" | cut -c 5-
}

opts::preview_hack() {
   local -r arg="$1"

   if [ ${arg:0:1} = "'" ]; then
      echo "${arg:1:${#arg}-2}"
   else
      echo "$arg"
   fi
}

opts::eval() {
   local wait_for=""

   entry_point="main"
   print=false
   interpolation=true
   preview=true

   for arg in "$@"; do
      case $wait_for in
         path) NAVI_PATH="$arg"; wait_for="" ;;
         preview) query="$(opts::preview_hack "$arg" | tr "^" " ")"; wait_for=""; break ;;
         search) query="$arg"; wait_for=""; export NAVI_PATH="${NAVI_PATH}:$(search::full_path "$query")"; ;;
         query) query="$arg"; wait_for="" ;;
      esac

      case $arg in
         --print) print=true ;;
         --no-interpolation) interpolation=false ;;
         --version) echo "${VERSION:-unknown}" && exit 0 ;;
         help|--help) opts::extract_help "$0" && exit 0 ;;
         --command-for) wait_for="command-for" ;;
         --no-preview) preview=false ;;
         --path) wait_for="path" ;;
         search) entry_point="search"; wait_for="search" ;;
         preview) entry_point="preview"; wait_for="preview" ;;
         q|query) wait_for="query" ;;
      esac
   done
}

opts::fallback_path() {
   echo "${NAVI_DIR:-${SCRIPT_DIR}/cheats}"
}

export NAVI_PATH="${NAVI_PATH:-$(opts::fallback_path)}"