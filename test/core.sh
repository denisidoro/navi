#!/usr/bin/env bash

source "${SCRIPT_DIR}/src/main.sh"

test::success() {
   echo "Test passed!"
}

test::fail() {
   echo "Test failed..."
   exit 42
}

test::run() {
   echo
   echo "-> $1"
   shift
   eval "$*" && test::success || test::fail
}