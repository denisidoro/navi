#!/usr/bin/env zsh

_call_navi() {
   local -r buff="$BUFFER"
   local -r f="$(mktemp || echo "${HOME}/.naviresult")" 2>/dev/null
   navi --save "$f" </dev/tty >/dev/tty
   local -r r="$(cat "$f")" 2>/dev/null
   rm "$f" 2> /dev/null || true
   zle kill-whole-line
   zle -U "${buff}${r}"
}

zle -N _call_navi

bindkey '^g' _call_navi
