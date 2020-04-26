#!/bin/bash

case "${snippet:-}" in
   *docker*|*osascript*|*Finder*|*open*) exit 0 ;;
esac

printf "terminal"
