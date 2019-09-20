#!/usr/bin/env bash

source <(
grep -v 'main ' "${SCRIPT_DIR}/navi" | sed -E "s|export.*|export SCRIPT_DIR=\"${SCRIPT_DIR}\"|")