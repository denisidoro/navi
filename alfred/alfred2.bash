#!/bin/bash

if [ -n "${varname:-}" ]; then
   source "${HOME}/.bashrc"
   echo -n "$(navi alfred transform)"
else
   echo -n "$snippet"
fi
