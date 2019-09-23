#!/usr/bin/env bash

source "${SCRIPT_DIR}/src/main.sh"

PASSED=0
FAILED=0

test::success() {
   PASSED=$((PASSED+1))
   echo "Test passed!"
}

test::fail() {
   FAILED=$((FAILED+1))
   echo "Test failed..."
}

test::run() {
   echo
   echo "-> $1"
   shift
   eval "$*" && test::success || test::fail
}

test::finish() {
   echo
   if [ $FAILED -gt 0 ]; then
      echo "${PASSED} tests passed but ${FAILED} failed... :("
      exit 99
   else
      echo "All ${PASSED} tests passed! :)"
      exit 0
   fi
}
