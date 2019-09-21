#!/usr/bin/env bash

source "${SCRIPT_DIR}/src/main.sh"

test::success() {
   echo "Test passed!"
}

test::fail() {
   echo "Test failed..."
   exit 42
}