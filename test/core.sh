#!/usr/bin/env bash

source <(
grep -v 'main ' "${SCRIPT_DIR}/navi" | sed -E "s|export.*|export SCRIPT_DIR=\"${SCRIPT_DIR}\"|")

test::success() {
   echo "Test passed!"
}

test::fail() {
   echo "Test failed..."
   exit 42
}