#!/usr/bin/env bash
set -euo pipefail

opts::extract_help() {
   local -r file="${NAVI_HOME}/docstring.txt"
   cat "$file"
}

opts::eval() {
   local wait_for=""
   local entry_point="main"
   local print=false
   local interpolation=true
   local preview=true
   local path="${NAVI_PATH:-${NAVI_DIR:-${NAVI_HOME}/cheats}}"
   local autoselect=true
   local best=false
   local query=""
   local values=""
   local col_widths="15,50,0"
   local col_colors="90,34,37"
   local fzf_overrides="--with-nth 3,1,2 --exact"
   local fzf_opts="${FZF_DEFAULT_OPTS:---height 70% --reverse --border --inline-info --cycle}"

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
      fn) entry_point="fn"; shift; SCRIPT_ARGS="$@" ;;
      widget) entry_point="widget"; shift; wait_for="widget" ;;
   esac

   for arg in "$@"; do
      case $wait_for in
         path) path="$arg"; wait_for=""; continue ;;
         preview) query="$(arg::deserialize "$arg")"; wait_for=""; continue ;;
         search) query="$arg"; wait_for=""; path="${path}:$(search::full_path "$query")"; continue ;;
         query|best) query="$arg"; wait_for=""; continue ;;
         widget) SH="$arg"; wait_for=""; continue ;;
         col-widths) col_widths="$(echo "$arg" | xargs | tr ' ' ',')"; wait_for=""; continue ;;
         col-colors) col_colors="$(echo "$arg")"; wait_for=""; continue ;;
         fzf-overrides) fzf_overrides="$arg" ; wait_for=""; continue ;;
      esac

      case $arg in
         --print) print=true ;;
         --no-interpolation) interpolation=false ;;
         --interpolation) interpolation=true ;;
         --no-preview) preview=false ;;
         --preview) preview=true ;;
         --path|--dir) wait_for="path" ;;
         --no-autoselect) autoselect=false ;;
         --autoselect) autoselect=true ;;
         --col-widths) wait_for="col-widths" ;;
         --col-colors) wait_for="col-colors" ;;
         --fzf-overrides) wait_for="fzf-overrides" ;;
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
      values "$values" \
      fzf-overrides "$fzf_overrides" \
      col-colors "$col_colors" \
      col-widths "$col_widths")"

   export NAVI_PATH="$path"
   export FZF_DEFAULT_OPTS="$fzf_opts"
}
