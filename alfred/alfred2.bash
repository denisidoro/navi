#!/bin/bash

_interpolate() {
        local -r snippet="$1"
        local -r varname="$2"
        local -r value="${!varname}"

        echo "$snippet" | sed "s/<${varname}>/${value}/g"
}

if [ -n "${varname:-}" ]; then 
        echo -n "$(_interpolate "$snippet" "$varname" || echo "")"
else
        echo -n "$snippet"
fi
