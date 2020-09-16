#!/usr/bin/env bash
set -euo pipefail

source "${NAVI_HOME}/scripts/aux/tests.sh"

_navi_tldr() {
   _navi --tldr docker --query ps --print --best-match \
      | test::equals "docker ps"
}

_navi_cheatsh() {
   _navi --cheatsh docker --query remove --print --best-match \
      | test::equals "docker rm container_name"
}

test::set_suite "3rd party"
test::run "tldr" _navi_tldr
test::run "cheatsh" _navi_cheatsh