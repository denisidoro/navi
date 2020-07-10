#!/bin/bash

source "${HOME}/.bashrc"
export PATH="/usr/local/bin:$PATH"

if [ -n "${snippet:-}" ]; then
   echo -n "$(navi alfred check)"
else
   echo -n "__start"
fi
