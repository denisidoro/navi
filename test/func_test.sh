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

powers() {
   local x="$1"
   func::list $((x*10)) $((x*100))
}

odd() {
   local x="$1"
   [ $((x%2)) -eq 1 ]
}

func_map() {
   func::list 1 2 3 \
      | func::map inc \
      | test::equals "$(func::list 2 3 4)"
}

func_flatmap() {
   func::list 1 2 3 \
      | func::map powers \
      | test::equals "$(func::list 10 100 20 200 30 300)"
}

func_reduce() {
   func::list 1 2 3 \
      | func::reduce sum 10 \
      | test::equals 60
}

func_filter() {
   func::list 1 2 3 4 5 \
      | func::filter odd \
      | test::equals "$(func::list 1 3 5)"
}

func_remove() {
   func::list 1 2 3 4 5 \
      | func::remove odd \
      | test::equals "$(func::list 2 4)"
}

test::run "map works" func_map
test::run "filter works" func_filter
test::run "remove works" func_remove
test::run "reduce works" func_reduce
test::run "we can use map as flatmap" func_flatmap
