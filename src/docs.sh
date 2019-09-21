#!/usr/bin/env bash
set -euo pipefail

docs::extract_help() {
   local readonly file="$1"
   grep "^##?" "$file" | cut -c 5-
}

docs::eval() {
   local wait_for=""

   entry_point="main"
   print=false
   interpolation=true
   preview=true

   for arg in $@; do
      case $wait_for in
         dir) NAVI_DIR="$arg"; wait_for="" ;;
         command-for) query="$(echo "$arg" | tr "^" " ")"; entry_point="preview"; break ;;
      esac

      case $arg in
         --print) print=true ;;
         --no-interpolation) interpolation=false ;;
         --version) echo "${VERSION:-unknown}" && exit 0 ;;
         --help) docs::extract_help "$0" && exit 0 ;;
         --command-for) wait_for="command-for" ;;
         --no-preview) preview=false ;;
         -d|--dir) wait_for="dir" ;;
      esac
   done
}
