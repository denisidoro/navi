#!/usr/bin/env bash

health::fzf() {
   if ! command_exists fzf; then
      echoerr "You need to install fzf before using navi"
      echoerr "Please refer to https://github.com/junegunn/fzf for install instructions"
      exit 66
   fi
}