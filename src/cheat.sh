#!/usr/bin/env bash

cheat::find() {
	find "${cheat_dir:-"${SCRIPT_DIR}/cheats"}" -iname '*.cheat'
}

cheat::read_many() {
    for cheat in $(cat); do
        awk '
        function color(c,s) {
           printf("\033[%dm%s\033[0m",30+c,s)
        }
        
        /^%/ { tags=" ["substr($0, 3)"]"; next }
        /^#/ { print color(4, $0) color(7, "\033[2m"tags); next }
        /^\$/ { next }
        NF { print color(7, $0) color(7, "\033[2m"tags); next }' "$cheat"
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
