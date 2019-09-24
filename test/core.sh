#!/usr/bin/env bash

source "${SCRIPT_DIR}/src/main.sh"
source "${SCRIPT_DIR}/test/log.sh"

opts::eval "$@"

PASSED=0
FAILED=0

test::success() {
   PASSED=$((PASSED+1))
   log::success "Test passed!"
}

test::fail() {
   FAILED=$((FAILED+1))
   log::error "Test failed..."
   return
}

test::run() {
   echo
   log::note "$1"
   shift
   eval "$*" && test::success || test::fail
}

test::equals() {
   local -r actual="$(cat | tr -d '\n')"
   local -r expected="$(echo "${1:-}" | tr -d '\n' | sed 's/\\n//g')"

   if [[ "$actual" != "$expected" ]]; then
      log::success "Expected '${expected}' but got '${actual}'"
      return 2
   fi
}

test::finish() {
   echo
   if [ $FAILED -gt 0 ]; then
      log::error "${PASSED} tests passed but ${FAILED} failed... :("
      exit "${FAILED}"
   else
      log::success "All ${PASSED} tests passed! :)"
      exit 0
   fi
}
