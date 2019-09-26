#!/usr/bin/env bash

test::coll_equals() {
   local -r actual="$(cat)"
   local -r expected="$(coll::new "$@")"

   echo "$actual" | test::equals "$expected"
}

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
   coll::new $((x*10)) $((x*100))
}

odd() {
   local x="$1"
   [ $((x%2)) -eq 1 ]
}

coll_map() {
   coll::new 1 2 3 \
      | coll::map inc \
      | test::coll_equals 2 3 4
}

coll_flatmap() {
   coll::new 1 2 3 \
      | coll::map powers \
      | test::coll_equals 10 100 20 200 30 300
}

coll_reduce() {
   coll::new 1 2 3 \
      | coll::reduce sum 10 \
      | test::equals 60
}

coll_filter() {
   coll::new 1 2 3 4 5 \
      | coll::filter odd \
      | test::coll_equals 1 3 5
}

coll_remove() {
   coll::new 1 2 3 4 5 \
      | coll::remove odd \
      | test::coll_equals 2 4
}

coll_first() {
   coll::new 1 2 3 \
      | coll::first \
      | test::coll_equals 1
}

coll_rest() {
   coll::new 1 2 3 \
      | coll::rest \
      | test::coll_equals 2 3
}

coll_add() {
   coll::new 1 2 3 \
      | coll::add 4 5 \
      | coll::add 6 7 \
      | test::coll_equals 1 2 3 4 5 6 7
}

coll_concat() {
   coll::new 1 2 3 \
      | coll::add "$(coll::new 4 5)" \
      | test::coll_equals 1 2 3 4 5
}

coll_reverse() {
   coll::new 1 2 3 \
      | coll::reverse \
      | test::coll_equals 3 2 1
}

coll_set() {
   coll::new 1 2 3 2 4 2 \
      | coll::set \
      | test::coll_equals 1 2 3 4
}

test::set_suite "coll"
test::run "map" coll_map
test::run "filter" coll_filter
test::run "remove" coll_remove
test::run "first" coll_first
test::run "rest" coll_rest
test::run "add" coll_add
test::run "add can be used as concat" coll_concat
test::run "reduce" coll_reduce
test::run "we can use map as flatmap" coll_flatmap
test::run "reverse" coll_reverse
test::run "set" coll_set
