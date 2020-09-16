#!/usr/bin/env bash

DOTFILES_COMMIT_HASH="bc74f8"

dot::clone() {
   git clone "https://github.com/denisidoro/dotfiles.git" "$DOTFILES"
   cd "$DOTFILES" && git checkout "$DOTFILES_COMMIT_HASH"
}

dot::install_if_necessary() {
   [ -n "${DOTFILES:-}" ] && [ -f "${DOTFILES}/bin/dot" ] && return
   export DOTFILES="${NAVI_HOME}/dotfiles"
   $(dot::clone 2>/dev/null || true)
}

export NAVI_HOME="${NAVI_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"

export PROJ_HOME="$NAVI_HOME"
export PROJ_NAME="navi"

dot::install_if_necessary
export PATH="${DOTFILES}/bin:${PATH}"

source "${DOTFILES}/scripts/core/main.sh"

export NAVI_BIN="${NAVI_HOME}/target/release/NAVI"
[ -f "$NAVI_BIN" ] || export NAVI_BIN="${NAVI_HOME}/target/debug/navi"
[ -f "$NAVI_BIN" ] || export NAVI_BIN="${NAVI_HOME}/scripts/run"