#!/usr/bin/env zsh

_navi_call() {
	 local -r query="$1"
	 shift
	 if [ -n "$query" ]; then
	    set -- "$@" "$query"
	 fi
    local result="$(navi "$@" </dev/tty)"
    if [ -z "${result}" ] && [ -n "$query" ]; then
        result="$(navi --print </dev/tty)"
    fi
    printf "%s" "$result"
}

_navi_widget() {
    local -r input="${LBUFFER}"
    local -r last_command="$(echo "${input}" | navi fn widget::last_command)"
    local find="$last_command"
    local replacement="$last_command"

    if [ -z "${last_command}" ]; then 
        replacement="$(FZF_OVERRIDES="${FZF_OVERRIDES:-}" _navi_call "" --print)"
    elif [ "${LASTWIDGET}" = "_navi_widget" ] && [ "$input" = "$previous_output" ]; then
        find="$input"
        replacement="$(_navi_call "${previous_last_command:-$last_command}" --print --query)"
    else
        replacement="$(_navi_call "${previous_last_command:-$last_command}" --print --best-match --query)"
    fi

    previous_last_command="$last_command"
    previous_output="${input//$find/$replacement}"

    zle kill-whole-line
    LBUFFER="${previous_output}"
    region_highlight=("P0 100 bold")
    zle redisplay
}

zle -N _navi_widget
bindkey '^g' _navi_widget
