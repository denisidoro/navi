#!/usr/bin/env bash
set -euo pipefail

source "${NAVI_HOME}/scripts/aux/tests.sh"

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

_navi_cases() {
   local filter="$*"
   filter="${filter::-2}"
   _navi --query "$filter" --best-match
}

_navi_cases_test() {
   _navi_cases "$1" \
      | test::loose_equals "$2"
}

_get_all_tests() {
   cat "${TEST_CHEAT_PATH}/cases.cheat" \
      | grep '^#' \
      | grep ' ->' \
      | sed 's/\\n/ /g' \
      | sed -E 's/# (.*) -> "(.*)"/\1|\2/g'
}

test::set_suite "cases"
ifs="$IFS"
IFS=$'\n'
for i in $(_get_all_tests); do
   IFS="$ifs"
   query="$(echo "$i" | cut -d'|' -f1)"
   expected="$(echo "$i" | cut -d'|' -f2)"
   test::run "$query" _navi_cases_test "$query" "$expected"
done
