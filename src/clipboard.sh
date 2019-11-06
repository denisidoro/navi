#!/usr/bin/env bash

clip::set() {
   local -r input="${1:-}"

   if command_exists pbcopy; then
      echo "$input" | pbcopy
   elif command_exists clip.exe; then
      echo "$input" | clip.exe
   elif command_exists xclip; then
      echo "$input" | xclip -sel clip
   elif command_exists xsel; then
      echo "$input" | xsel -i -b
   else
      echo "$input"
      die "Unable to set clipboard"
   fi
}