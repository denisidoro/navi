#!/usr/bin/env bash

health::fzf() {
   if ! command_exists ui::fzf && [ $NAVI_ENV != "test" ]; then
      echoerr "You need to install a fuzzy finder before using navi. Refer to the following for install instructions."
      echoerr "  fzf - https://github.com/junegunn/fzf"
      echoerr "  sk - https://github.com/lotabout/skim"
      exit 66
   fi
}
