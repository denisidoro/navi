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
   local autoselect=true
   local best=false
   local query=""
   local values=""

   case "${1:-}" in
      --version|version) entry_point="version"; shift ;;
      --full-version|full-version) entry_point="full-version"; shift ;;
      --help|help) entry_point="help"; shift ;;
      search) entry_point="search"; wait_for="search"; shift ;;
      preview) entry_point="preview"; wait_for="preview"; shift ;;
      query|q) wait_for="query"; shift ;;
      best|b) best=true; wait_for="best"; shift ;;
      home) entry_point="home"; shift ;;
      script) entry_point="script"; shift; SCRIPT_ARGS="$@" ;;
      widget) entry_point="widget"; shift; wait_for="widget" ;;
   esac

   for arg in "$@"; do
      case $wait_for in
         path) path="$arg"; wait_for=""; continue ;;
         preview) query="$(arg::deserialize "$arg")"; wait_for=""; continue ;;
         search) query="$arg"; wait_for=""; path="${path}:$(search::full_path "$query")"; continue ;;
         query|best) query="$arg"; wait_for=""; continue ;;
         widget) SH="$arg"; wait_for=""; continue ;;
      esac

      case $arg in
         --print) print=true ;;
         --no-interpolation) interpolation=false ;;
         --no-preview) preview=false ;;
         --path|--dir) wait_for="path" ;;
         --no-autoselect) autoselect=false ;;
         *) values="$(echo "$values" | coll::add "$arg")" ;;
      esac
   done

   OPTIONS="$(dict::new \
      entry_point "$entry_point" \
      print "$print" \
      interpolation "$interpolation" \
      preview "$preview" \
      autoselect "$autoselect" \
      query "$query" \
      best "$best" \
      values "$values")"

   export NAVI_PATH="$path"
}
