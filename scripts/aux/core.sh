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

fzf::export_if_necessary() {
   if ! has fzf; then
      export PATH="$PATH:$HOME/.fzf/bin"
   fi
}

export NAVI_HOME="${NAVI_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"

export PROJ_HOME="$NAVI_HOME"
export PROJ_NAME="navi"

dot::install_if_necessary

source "${DOTFILES}/scripts/core/main.sh"
source "${DOTFILES}/scripts/core/log.sh"

fzf::export_if_necessary
export PATH="${NAVI_HOME}/scripts:${DOTFILES}/bin:${PATH}"

cd "$NAVI_HOME"