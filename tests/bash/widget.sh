#!/usr/bin/env bash
set -euo pipefail

source "${NAVI_HOME}/scripts/aux/tests.sh"

_navi_widget() {
   _navi widget "$1" \
      | grep -q "#!/usr/bin/env $1"
}

test::set_suite "widget"
test::run "bash" _navi_widget "bash"
test::run "zsh" _navi_widget "zsh"
test::run "zsh" _navi_widget "fish"