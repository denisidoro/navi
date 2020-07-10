#!/bin/bash

source "${HOME}/.bashrc"
export PATH="/usr/local/bin:$PATH"

_hack() {
   sed $'s,\x1b\\[[0-9;]*[a-zA-Z],,g'
}

if [ -n "${snippet:-}" ]; then
   navi alfred suggestions | _hack
else
   navi alfred start | _hack
fi
