#!/usr/bin/env bash

ui::pick() {
    fzf --inline-info "$@"
}

ui::select() {
    local readonly cheats="$1"

    echo "$cheats" \
        | cheat::read_many \
        | ui::pick -i --ansi --delimiter '\^' --with-nth 2
}

ui::clear_previous_line() {
    tput cuu1 && tput el || true
}
