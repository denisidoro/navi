#!/usr/bin/env bash
set -euo pipefail

docs::extract_help() {
   local -r file="$1"
   grep "^##?" "$file" | cut -c 5-
}

docs::eval() {
   local wait_for=""

   print=false
   interpolation=true

   for arg in "$@"; do
      case $wait_for in
         dir) NAVI_DIR="$arg"; wait_for="" ;;
      esac

      case $arg in
         --print) print=true ;;
         --no-interpolation) interpolation=false ;;
         --version) echo "${VERSION:-unknown}" && exit 0 ;;
         --help) docs::extract_help "$0" && exit 0 ;;
         -d|--dir) wait_for="dir" ;;
      esac
   done
}
