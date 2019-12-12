#!/usr/bin/env bash

assert_docker_cheat() {
   cheat::find | test::contains "docker.cheat"
}

test::set_suite "cheat"
test::run "We can find at least one known cheatsheet" assert_docker_cheat
