#!/usr/bin/env bash

existing() {
   platform::existing_command oasida fngo ni awk aoisdn oafm \
    | test::equals awk
}

exit_code() {
  "$@"
  echo $?
}

exists() {
   platform::command_exists awk \
    && exit_code \
    || exit_code \
    | assert 
}

test::set_suite "platform"
test::run "existing_command" existing
test::run "command_exists" exists
