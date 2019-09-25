#!/usr/bin/env bash

inc() {
   local -r x="$1"
   echo $((x+1))
}

sum() {
   local -r x="$1"
   local -r y="$2"
   echo $((x*y))
}

func_map() {
   func::list 1 2 3 \
      | func::map inc \
      | test::equals "$(func::list 2 3 4)"
}

func_reduce() {
   func::list 1 2 3 \
      | func::reduce sum 10 \
      | test::equals 60
}

test::run "map works" func_map
test::run "reduce works" func_reduce
