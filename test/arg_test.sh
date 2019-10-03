#!/usr/bin/env bash

interpolation_one_word() {
   echo "curl http://mysite.com/<user>/profile" \
      | arg::interpolate "user" "john" \
      | test::equals "curl http://mysite.com/john/profile"
}

interpolation_multiple_words() {
   echo "cp <file> <new_file>" \
      | arg::interpolate "file" "C:/Program Files/app/foo.exe" \
      | arg::interpolate "new_file" "/mnt/c/Users/john/foo.exe" \
      | test::equals 'cp "C:/Program Files/app/foo.exe" /mnt/c/Users/john/foo.exe'
}

test::set_suite "arg"
test::run "if there's only one word, interpolation doesn't include quotes" interpolation_one_word
test::run "if there are multiple words, interpolation includes quotes" interpolation_multiple_words
