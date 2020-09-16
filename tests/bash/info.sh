#!/usr/bin/env bash
set -euo pipefail

source "${NAVI_HOME}/scripts/aux/tests.sh"

_navi_cheatspath() {
   _navi info cheats-path \
      | grep -q "/cheats"
}

test::set_suite "info"
test::run "cheats_path" _navi_cheatspath
