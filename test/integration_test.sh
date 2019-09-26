#!/usr/bin/env bash

NAVI_BIN="${SCRIPT_DIR}/navi"
TEST_DIR="${SCRIPT_DIR}/test"

_navi() {
   "$NAVI_BIN" "$@"
}

fzf_mock() {
   head -n1 | sed 's/\x1b\[[0-9;]*m//g'
}

assert_version() {
   local -r version="$(cat "$NAVI_BIN" | grep VERSION | cut -d'=' -f2 | tr -d '"')"

   _navi --version \
      | test::equals "$version"
}

assert_help() {
   _navi --help \
      | grep -q 'Options:'
}

assert_home() {
   _navi home \
      | grep -q '/'
}

assert_best() {
   _navi best constant --path "$TEST_DIR" \
      | test::equals 42
}

assert_query() {
   NAVI_ENV="test" _navi --path "$TEST_DIR" \
      | test::equals "2 12"
}

test::set_suite "integration"
export -f fzf_mock
test::run "version" assert_version
test::run "help" assert_help
test::run "home" assert_home
test::skip "best" assert_best # FZF setup needed in CircleCI
test::skip "query" assert_query # FZF setup needed in CircleCI
