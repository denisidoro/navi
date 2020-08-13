#!/usr/bin/env bash

_call_navi() {
    local selected

    if [ -n "${READLINE_LINE}" ]; then
        if selected="$(printf "%s" "$(navi --print --no-autoselect query "${READLINE_LINE}" </dev/tty)")"; then
            READLINE_LINE="$selected"
            READLINE_POINT=${#READLINE_LINE}
        fi
    else
        if selected="$(printf "%s" "$(navi --print </dev/tty)")"; then
            READLINE_LINE="$selected"
            READLINE_POINT=${#READLINE_LINE}
        fi
    fi
}

bind -x '"\C-g": _call_navi'
