#!/usr/bin/env bash

_export_colors() {
   if ! ${DOT_COLORS_EXPORTED:-false}; then
      if [ -z ${TERM:-} ] || [ $TERM = "dumb" ]; then
         bold=""
         underline=""
         freset=""
         purple=""
         red=""
         green=""
         tan=""
         blue=""
      else
         bold=$(tput bold)
         underline=$(tput sgr 0 1)
         freset=$(tput sgr0)
         purple=$(tput setaf 171)
         red=$(tput setaf 1)
         green=$(tput setaf 76)
         tan=$(tput setaf 3)
         blue=$(tput setaf 38)
      fi

      log_black=30
      log_red=31
      log_green=32
      log_yellow=33
      log_blue=34
      log_purple=35
      log_cyan=36
      log_white=37

      log_regular=0
      log_bold=1
      log_underline=4

      readonly DOT_COLORS_EXPORTED=true
   fi
}

log::color() {
   _export_colors
   local bg=false
   case "$@" in
      *reset*) echo "\e[0m"; exit 0 ;;
      *black*) color=$log_black ;;
      *red*) color=$log_red ;;
      *green*) color=$log_green ;;
      *yellow*) color=$log_yellow ;;
      *blue*) color=$log_blue ;;
      *purple*) color=$log_purple ;;
      *cyan*) color=$log_cyan ;;
      *white*) color=$log_white ;;
   esac
   case "$@" in
      *regular*) mod=$log_regular ;;
      *bold*) mod=$log_bold ;;
      *underline*) mod=$log_underline ;;
   esac
   case "$@" in
      *background*) bg=true ;;
      *bg*) bg=true ;;
   esac

   if $bg; then
      echo "\e[${color}m"
   else
      echo "\e[${mod:-$log_regular};${color}m"
   fi
}

if [ -z ${LOG_FILE+x} ]; then
   readonly LOG_FILE="/tmp/$(basename "$0").log"
fi

_log() {
   local template=$1
   shift
   if ${log_to_file:-false}; then
      echo -e $(printf "$template" "$@") | tee -a "$LOG_FILE" >&2
   else
      echo -e $(printf "$template" "$@")
   fi
}

_header() {
   local TOTAL_CHARS=60
   local total=$TOTAL_CHARS-2
   local size=${#1}
   local left=$((($total - $size) / 2))
   local right=$(($total - $size - $left))
   printf "%${left}s" '' | tr ' ' =
   printf " $1 "
   printf "%${right}s" '' | tr ' ' =
}

log::header() { _export_colors && _log "\n${bold}${purple}$(_header "$1")${freset}\n"; }
log::success() { _export_colors && _log "${green}✔ %s${freset}\n" "$@"; }
log::error() { _export_colors && _log "${red}✖ %s${freset}\n" "$@"; }
log::warning() { _export_colors && _log "${tan}➜ %s${freset}\n" "$@"; }
log::note() { _export_colors && _log "${blue}➜ %s${freset}\n" "$@"; }
