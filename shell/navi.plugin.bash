#!/usr/bin/env bash

_call_navi() {
    local selected

    if [ -n "${READLINE_LINE}" ]; then
        if selected="$(printf "%s" "$(navi --print --fzf-overrides '--no-select-1' --query "${READLINE_LINE}" </dev/tty)")"; then
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

__call_navi_legacy_versions() {
    local -r result="$(navi --print)"
    local -r linecount="$(echo "$result" | wc -l)"

    if [[ "$linecount" -lt 2 ]]; then
        printf "%s" "$result"
        return 0
    fi

    IFS=$'\n'
    local i=1;
    for line in $result; do
        if echo "$line" | grep -q '\\$'; then
            printf "${line::-1} "
        elif [[ "$i" -eq "$linecount" ]]; then
            printf "$line "
        else 
            printf "${line}; "
        fi
        i=$((i+1))
    done
}

if [ ${BASH_VERSION:0:1} -lt 4 ]; then 
    bind '"\C-g": " \C-b\C-k \C-u`__call_navi_legacy_versions`\e\C-e\C-a\C-y\C-h\C-e\e \C-y\ey\C-x\C-x\C-f"'
else
    bind -x '"\C-g": _call_navi'
fi
