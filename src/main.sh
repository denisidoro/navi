#!/usr/bin/env bash
set -euo pipefail

main() {
    local readonly cheats="$(cheat::find)"
    local readonly selection="$(ui::select "$cheats")"
    local readonly cheat="$(cheat::from_selection "$cheats" "$selection")"
    local cmd="$(selection::command "$selection" "$cheat")"
    local arg value

    if ! $interpolation; then
        echo "$cmd"
        exit 0
    fi

    while true; do
        arg="$(echo "$cmd" | arg::next || echo "")"
        if [ -z "$arg" ]; then 
            break
        fi

        value="$(arg::pick "$arg" "$cheat" || echo "")"
        if [ -z "$value" ]; then
            echo "$cmd"
            exit 0
        fi

        eval "local $arg"='$value'
        cmd="$(echo "$cmd" | arg::interpolate "$arg" "$value")"
    done

    if $print; then
        echo "$cmd"
    else
        eval "$cmd"
    fi
}
