#!/bin/bash

source "${HOME}/.bashrc"
export PATH="/usr/local/bin:$PATH"

if [ -n "${varname:-}" ]; then
   echo -n "$(navi alfred transform)" | sed 's/   Needs Review       Add new hash function to predicate-eval//' | tr -d '\n'
else
   echo -n "$snippet"
fi
