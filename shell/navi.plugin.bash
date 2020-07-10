#!/usr/bin/env bash

__call_navi() {
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

bind '"\C-g": " \C-b\C-k \C-u`__call_navi`\e\C-e\C-a\C-y\C-h\C-e\e \C-y\ey\C-x\C-x\C-f"'