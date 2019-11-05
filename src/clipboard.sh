#!/usr/bin/env bash

clip::cmd() {
   if command_exists pbcopy; then
      echo pbcopy
   elif command_exists xclip; then
      echo xclip -sel clip
   elif command_exists xsel; then
      echo xsel -i -b
   fi
}