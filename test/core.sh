#!/usr/bin/env bash

source "${SCRIPT_DIR}/src/main.sh"

opts::eval "$@"
export NAVI_PATH="$(dict::get "$OPTIONS" path)"

PASSED=0
FAILED=0

test::success() {
   PASSED=$((PASSED+1))
   echo "Test passed!"
}

test::fail() {
   FAILED=$((FAILED+1))
   echo "Test failed..."
   return
}

test::run() {
   echo
   echo "-> $1"
   shift
   eval "$*" && test::success || test::fail
}

test::equals() {
   local -r actual="$(cat | tr -d '\n')"
   local -r expected="$(echo "${1:-}" | tr -d '\n' | sed 's/\\n//g')"

   if [[ "$actual" != "$expected" ]]; then
      echo "Expected '${expected}' but got '${actual}'"
      return 2
   fi
}

test::finish() {
   echo
   if [ $FAILED -gt 0 ]; then
      echo "${PASSED} tests passed but ${FAILED} failed... :("
      exit "${FAILED}"
   else
      echo "All ${PASSED} tests passed! :)"
      exit 0
   fi
}
