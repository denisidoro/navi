#!/usr/bin/env bash

_navi_call() {
    local result="$(navi "$@" </dev/tty)"
    if [ -z "${result}" ]; then
        result="$(navi --print </dev/tty)"
    fi
    printf "%s" "$result"
}

_navi_widget() {
    local -r input="${READLINE_LINE}"
    local -r last_command="$(echo "${input}" | navi fn widget::last_command)"

    if [ -z "${last_command}" ]; then 
        local -r output="$(_navi_call --print --fzf-overrides '--no-select-1')"
    else
        local -r find="$last_command"
        local -r replacement="$(_navi_call --print --query "${last_command}")"
        local -r output="${input//$find/$replacement}"
    fi

    READLINE_LINE="$output"
    READLINE_POINT=${#READLINE_LINE}
}

_navi_widget_legacy() {
    _navi_call --print
}

if [ ${BASH_VERSION:0:1} -lt 4 ]; then 
    bind '"\C-g": " \C-b\C-k \C-u`_navi_widget_legacy`\e\C-e\C-a\C-y\C-h\C-e\e \C-y\ey\C-x\C-x\C-f"'
else
    bind -x '"\C-g": _navi_widget'
fi