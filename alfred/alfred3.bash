#!/bin/bash

case "${snippet:-}" in
   *docker*|*osascript*|*open*) exit 0 ;;
esac

printf "terminal"
