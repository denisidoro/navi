_call_navi() {
   # zle kill-whole-line
   zle -U "$(NAVI_FZF_ALL_INPUTS=true navi --print "$(echo "$CUTBUFFER" | xargs)" <> /dev/tty)"
   zle accept-line
}

zle -N _call_navi

bindkey '\eg' _call_navi