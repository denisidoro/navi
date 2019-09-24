#!/usr/bin/env bash

assert_docker_cheat() {
   cheat::find | grep -q "docker.cheat"
}

test::run "We can find at least one known cheatsheet" assert_docker_cheat
