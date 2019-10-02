local _navi_path=$(dirname $0:A)

_call_navi() {
   local buff="$BUFFER"
   zle kill-whole-line
   local cmd="$(NAVI_USE_FZF_ALL_INPUTS=true $_navi_path/navi --print <> /dev/tty)"
   zle -U "${buff}${cmd}"
   # zle accept-line
}

zle -N _call_navi

bindkey '\eg' _call_navi
