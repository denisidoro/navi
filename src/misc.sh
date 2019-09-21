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
