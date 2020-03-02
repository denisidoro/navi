#!/usr/bin/env bash

bind '"\C-g": " \C-u \C-a\C-k`printf \"\\e\" && NAVI_USE_FZF_ALL_INPUTS=true navi --print`\e\C-e\C-y\C-a\C-d\C-y\ey\C-h\C-e\C-b"'
