#!/bin/bash

source "${HOME}/.bashrc"

if [ -n "${varname:-}" ]; then
   echo -n "$(navi alfred transform)"
else
   echo -n "$snippet"
fi
