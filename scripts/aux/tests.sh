#!/usr/bin/env bash

source "${NAVI_HOME}/scripts/aux/core.sh"

export TEST_CHEAT_PATH="${NAVI_HOME}/tests/no_prompt_cheats"

test::loose_equals() {
   local -r actual="$(xargs | sed -E 's/\s/ /g')"
   local -r expected="$(echo "$1" | xargs | sed -E 's/\s/ /g')"
   echo "$actual" | test::equals "$expected"
}

_navi() {
   stty sane || true
   RUST_BACKTRACE=1 NAVI_PATH="${NAVI_TEST_PATH:-$TEST_CHEAT_PATH}" navi "$@"
}
