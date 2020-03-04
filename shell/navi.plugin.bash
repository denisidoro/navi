#!/usr/bin/env bash

__call_navi() {
    local -r f="$(mktemp || echo "${HOME}/.naviresult")"
    navi  --save "$f" </dev/tty >/dev/tty
    local -r r="$(cat "$f")"
    rm "$f" 2> /dev/null || true
    echo "$r"
}

bind '"\C-g": " \C-b\C-k \C-u`__call_navi`\e\C-e\C-a\C-y\C-h\C-e\e \C-y\ey\C-x\C-x\C-f"'