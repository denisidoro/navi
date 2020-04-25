#!/bin/bash

source "${HOME}/.bashrc"

if [ -n "${snippet:-}" ]; then
   navi alfred suggestions
else
   navi alfred start
fi
