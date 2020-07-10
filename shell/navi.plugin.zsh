#!/usr/bin/env zsh

_call_navi() {
  local selected
  if selected="$(printf "%s" "$(navi --print </dev/tty)")"; then
    LBUFFER="$selected"
  fi
  zle redisplay
}

zle -N _call_navi

bindkey '^g' _call_navi
