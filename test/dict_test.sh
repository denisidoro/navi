#!/usr/bin/env bash

inc() {
   local -r x="$1"
   echo $((x+1))
}

test::map_equals() {
   local -r actual="$(cat | dict::unescape_value | sort)"
   local -r expected="$(dict::new "$@" | dict::unescape_value | sort)"

   echo "$actual" | test::equals "$expected"
}

dict_assoc() {
   dict::new \
      | dict::assoc "foo" "42" \
      | tr -d "$ESCAPE_CHAR" \
      | tr -d "$ESCAPE_CHAR_2" \
      | tr -d "$ESCAPE_CHAR_3" \
      | test::equals "foo: 42"
}

dict_assoc_perc() {
   dict::new \
      | dict::assoc "foo" "42 %" bar "% 7" \
      | dict::get bar \
      | test::equals "% 7"
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
      | test::equals "$(echo -e "foo\nbar.a\nbar.b\nbaz")"
}

dict_get_values() {
   dict::new \
      | dict::assoc "foo" "42" "bar.a" 5 "bar.b" 6 "baz" 63 \
      | dict::values \
      | test::equals "$(echo -e "5\n6\n42\n63")"
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

dict_merge() {
   dict::new "foo" 42 "bar" 5 \
      | dict::merge "$(dict::new "bar" 7 "lorem" "ipsum")" \
      | test::map_equals "foo" 42
}

test::set_suite "dict"
test::run "We can assoc a value" dict_assoc
test::skip "We can merge dicts" dict_merge
test::run "We can assoc values with %" dict_assoc_perc
test::run "We can assoc multiple values" dict_assoc_multiple
test::skip "We can assoc a nested value" dict_assoc_nested
test::run "We can dissoc a value" dict_dissoc
test::run "Associng the same value is a no-op" dict_assoc_again
test::run "Dissocing a key will replace all its subvalues" dict_dissoc_nested
test::run "We can get a value" dict_get
test::run "We can get a nested value" dict_get_nested
test::run "We can get a dictionary" dict_get_dict
test::run "We can get all keys" dict_get_keys
test::skip "We can get all values" dict_get_values
test::skip "We can get create a dict from a zipmap" dict_zipmap
test::skip "We can update a value" dict_update
