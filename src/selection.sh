#!/usr/bin/env bash

selection::core() {
    cut -d'^' -f2
}

selection::tags() {
    cut -d'^' -f1
}

selection::core_is_comment() {
    grep -qE '^#'
}

selection::command() {
    local readonly selection="$1"
    local readonly cheat="$2"

    local readonly core="$(echo $selection | selection::core)"

    if echo "$core" | selection::core_is_comment; then
        grep "$core" "$cheat" -A999 \
          | str::last_paragraph_line
    else
        echo "$core"
    fi
}
