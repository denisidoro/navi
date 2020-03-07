#!/usr/bin/env zsh

_call_navi() {
   local -r buff="$BUFFER"
   local -r r="$(navi --print)"
   zle kill-whole-line
   zle -U "${buff}${r}"
}

zle -N _call_navi

bindkey '^g' _call_navi
