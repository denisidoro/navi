#!/bin/bash

if [ -n "${snippet:-}" ]; then
   source "${HOME}/.bashrc"
   echo -n "$(navi alfred check)"
else
   echo -n "__start"
fi
