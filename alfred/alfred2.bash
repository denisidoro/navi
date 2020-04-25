#!/bin/bash

source "${HOME}/.bashrc"

if [ -n "${varname:-}" ]; then
   echo -n "$(navi transform)"
else
   echo -n "$snippet"
fi
