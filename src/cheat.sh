#!/usr/bin/env bash

cheat::find() {
	find "${cheat_folder:-"${DIR}/sheets"}" -iname '*.cheat'
}

cheat::read_many() {
    for cheat in $(cat); do
        awk '
        function color(c,s) {
           printf("\033[%dm%s\033[0m",30+c,s)
        }
        
        /^%/ { tags=substr($0, 3); next }
        /^#/ { print color(3, tags"^") color(4, $0); next }
        /^\$/ { next }
        NF { print color(3, tags"^") color(7, $0); next }' "$cheat"
    done
}

cheat::from_selection() {
    local readonly cheats="$1"
    local readonly selection="$2"
    
    local readonly tags="$(echo "$selection" | selection::tags)"

    for cheat in $cheats; do
        if grep -q "% $tags" "$cheat"; then
            echo "$cheat"
            break
        fi
    done
}
