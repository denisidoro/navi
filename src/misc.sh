#!/usr/bin/env bash

# no-op hack to set dependency order resolution
dep() {
   :
}

command_exists() {
   type "$1" &> /dev/null
}

echoerr() {
   echo "$@" 1>&2
}

url::open() {
  if command_exists xdg-open; then
    xdg-open "$@"
  elif command_exists open; then
    open "$@"
  elif command_exists google-chrome; then
    google-chrome "$@"
  elif command_exists firefox; then
    firefox "$@"
  fi
}