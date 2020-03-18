#!/usr/bin/env bash
# vim: filetype=sh

source "${NAVI_HOME}/scripts/install"

PASSED=0
FAILED=0
SKIPPED=0
SUITE=""

test::set_suite() {
   SUITE="$*"
}

test::success() {
   PASSED=$((PASSED+1))
   log::success "Test passed!"
}

test::fail() {
   FAILED=$((FAILED+1))
   log::error "Test failed..."
   return
}

test::skip() {
   echo
   log::note "${SUITE:-unknown} - ${1:-unknown}"
   SKIPPED=$((SKIPPED+1))
   log::warning "Test skipped..."
   return
}

test::run() {
   echo
   log::note "${SUITE:-unknown} - ${1:-unknown}"
   shift
   "$@" && test::success || test::fail
}

test::equals() {
   local -r actual="$(cat)"
   local -r expected="$(echo "${1:-}")"

   if [[ "$actual" != "$expected" ]]; then
      log::error "Expected '${expected}' but got '${actual}'"
      return 2
   fi
}

test::finish() {
   echo
   if [ $SKIPPED -gt 0 ]; then
      log::warning "${SKIPPED} tests skipped!"
   fi
   if [ $FAILED -gt 0 ]; then
      log::error "${PASSED} tests passed but ${FAILED} failed... :("
      exit "${FAILED}"
   else
      log::success "All ${PASSED} tests passed! :)"
      exit 0
   fi
}