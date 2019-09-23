#!/usr/bin/env bash

test::run "We can find at least one known cheatsheet" \
	'cheat::find | grep -q "docker.cheat"'
