#!/usr/bin/env bash

inc() {
   local -r x="$1"
   echo $((x+1))
}

test::map_equals() {
   local -r actual="$(cat | dict::_unescape_value | sort)"
   local -r expected="$(dict::new "$@" | dict::_unescape_value | sort)"

   if [[ "$actual" != "$expected" ]]; then
      log::error "Expected '${expected}' but got '${actual}'"
      return 2
   fi
}

dict_assoc() {
   dict::new \
      | dict::assoc "foo" "42" \
      | tr -d '\f' \
      | test::equals "foo: 42"
}

dict_assoc_multiple() {
   dict::new \
      | dict::assoc "foo" "42" "bar" "5" \
      | test::map_equals "bar" 5 "foo" 42
}

dict_dissoc() {
   dict::new \
      | dict::assoc "foo" "42" "bar" "5" \
      | dict::dissoc "bar" \
      | test::map_equals "foo" 42
}

dict_assoc_again() {
   dict::new \
      | dict::assoc "foo" "42" \
      | dict::assoc "foo" "42" \
      | test::map_equals "foo" 42
}

dict_dissoc_nested() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.b" 6 "baz" 63 \
      | dict::dissoc "bar" \
      | test::map_equals "baz" 63 "foo" 42
}

dict_assoc_nested() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.c" 7 "baz" 63 \
      | dict::assoc "bar.b" 6 \
      | dict::get "bar.b" \
      | test::equals "asdfsadf"
}

dict_get() {
   dict::new \
      | dict::assoc "foo" "42" \
      | dict::get "foo" \
      | test::equals "42"
}

dict_get_nested() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.b" 6 "baz" 63 \
      | dict::get "bar.a" \
      | test::equals "5"
}

dict_get_dict() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.b" 6 "baz" 63 \
      | dict::get "bar" \
      | test::map_equals "bar.a" 5 "bar.b" 6
}

dict_get_keys() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.b" 6 "baz" 63 \
      | dict::keys \
      | test::equals "bar.a\nbar.b\nbaz\nfoo"
}

dict_get_values() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.b" 6 "baz" 63 \
      | dict::values \
      | test::equals "5\n6\n63\n42"
}

dict_zipmap() {
   dict::zipmap "key1\nkey2\nkey3" "value1\nvalue2\nvalue3" \
      | test::map_equals "key1" "value1" "key2" "value2" "key3" "value3"
}

dict_update() {
   dict::new "foo" 42 "bar" 5 \
      | dict::update "bar" inc \
      | test::map_equals "foo" 42 "bar" 6
}

test::run "We can assoc a value" dict_assoc
test::run "We can assoc multiple values" dict_assoc_multiple
test::skip "We can assoc a nested value" dict_assoc_nested
test::run "We can dissoc a value" dict_dissoc
test::run "Associng the same value is a no-op" dict_assoc_again
test::run "Dissocing a key will replace all its subvalues" dict_dissoc_nested
test::run "We can get a value" dict_get
test::run "We can get a nested value" dict_get_nested
test::run "We can get a dictionary" dict_get_dict
test::skip "We can get all keys" dict_get_keys
test::skip "We can get all values" dict_get_values
test::skip "We can get create a dict from a zipmap" dict_zipmap
test::skip "We can update a value" dict_update
