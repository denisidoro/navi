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
   coll::new $((x*10)) $((x*100))
}

odd() {
   local x="$1"
   [ $((x%2)) -eq 1 ]
}

coll_map() {
   coll::new 1 2 3 \
      | coll::map inc \
      | test::equals "$(coll::new 2 3 4)"
}

coll_flatmap() {
   coll::new 1 2 3 \
      | coll::map powers \
      | test::equals "$(coll::new 10 100 20 200 30 300)"
}

coll_reduce() {
   coll::new 1 2 3 \
      | coll::reduce sum 10 \
      | test::equals 60
}

coll_filter() {
   coll::new 1 2 3 4 5 \
      | coll::filter odd \
      | test::equals "$(coll::new 1 3 5)"
}

coll_remove() {
   coll::new 1 2 3 4 5 \
      | coll::remove odd \
      | test::equals "$(coll::new 2 4)"
}

coll_first() {
   coll::new 1 2 3 \
      | coll::first \
      | test::equals "$(coll::new 1)"
}

coll_rest() {
   coll::new 1 2 3 \
      | coll::rest \
      | test::equals "$(coll::new 2 3)"
}

coll_add() {
   coll::new 1 2 3 \
      | coll::add 4 5 \
      | coll::add 6 7 \
      | test::equals "$(coll::new 1 2 3 4 5 6 7)"
}

coll_concat() {
   coll::new 1 2 3 \
      | coll::add "$(coll::new 4 5)" \
      | test::equals "$(coll::new 1 2 3 4 5)"
}

coll_reverse() {
   coll::new 1 2 3 \
      | coll::reverse \
      | test::equals "$(coll::new 3 2 1)"
}

coll_set() {
   coll::new 1 2 3 2 4 2 \
      | coll::set \
      | test::equals "$(coll::new 1 2 3 4)"
}

test::run "map works" coll_map
test::run "filter works" coll_filter
test::run "remove works" coll_remove
test::run "first works" coll_first
test::run "rest works" coll_rest
test::run "add works" coll_add
test::run "add can be used as concat" coll_concat
test::run "reduce works" coll_reduce
test::run "we can use map as flatmap" coll_flatmap
test::run "reverse works" coll_reverse
test::run "set works" coll_set
