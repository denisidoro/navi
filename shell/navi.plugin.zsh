#!/usr/bin/env zsh

_navi_call() {
    local result="$(navi "$@" </dev/tty)"
    if [ -z "${result}" ]; then
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
        replacement="$(_navi_call --print --fzf-overrides '--no-select-1')"
    elif [ "${LASTWIDGET}" = "_navi_widget" ] && [ "$input" = "$previous_output" ]; then
        find="$input"
        replacement="$(_navi_call --print --query "${previous_last_command:-$last_command}")"
    else
        replacement="$(_navi_call --print --best-match --query "${last_command}")"
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
