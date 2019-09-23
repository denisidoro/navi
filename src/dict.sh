#!/usr/bin/env bash

dict::_post() {
  sed -E 's/; /\\n/g' | awk 'NF > 0' | sort
}

dict::new() {
  if [ $# = 0 ]; then 
    echo ""
  else
    echo "" | dict::assoc "$@"
  fi
}

dict::dissoc() {
  local -r key="$1"

  grep -Ev "^${key}[^:]*:" | dict::_post
}

dict::assoc() {
  local -r key="${1:-}"
  local -r value="${2:-}"
  local -r input="$(cat)"

  if [ -z $key ]; then
    printf "$input" | dict::_post
    return
  fi

  if [ -n "$input" ]; then
    local -r base="$(printf "$input" | dict::dissoc "$key"); "
  else
    local -r base="" 
  fi

  shift 2
  printf "${base}${key}: ${value}" | dict::_post | dict::assoc "$@" | dict::_post
}

dict::get() {
  if [ $# = 1 ]; then
    local -r input="$(cat)"
    local -r key="$1"
  else
    local -r input="$1"
    local -r key="$2"
  fi

  local -r prefix="${key}[^:]*: "
  local -r result="$(echo "$input" | grep -E "^${prefix}")"
  local -r matches="$(echo "$result" | wc -l || echo 0)"

  if [ $matches -gt 1 ]; then
    echo "$result"
  else
    echo "$result" | sed -E "s/${prefix}//"
  fi
}

dict::keys() {
  grep -Eo '^[^:]+: ' | sed 's/: //g'
}

dict::values() {
  awk -F':' '{$1=""; print $0}' | cut -c3-
}

dict::zipmap() {
  IFS='\n'

  local -r keys_str="$1"
  local -r values_str="$2"

  keys=()
  values=()
  for key in $keys_str; do
    keys+=("$key")
  done
  for value in $values_str; do
    values+=("$value")
  done

  for ((i=0; i<${#keys[@]}; ++i)); do
    if [ -n "${keys[i]}" ]; then
      echo "${keys[i]}: ${values[i]}"
    fi
  done
}

dict::update() {
  local -r key="$1"
  local -r fn="$2"
  local -r input="$(cat)"

  local -r value="$(echo "$input" | dict::get "$key")"
  local -r updated_value="$(eval "$fn" "$value")"

  echo "$input" | dict::assoc "$key" "$updated_value"
}