#!/bin/bash

setup() {
  echo "setup() is required"
  exit 1
}

cmd() {
  echo "cmd() is required"
  exit 1
}

assert() {
  echo "assert() is required"
  exit 1
}

name() {
  exit 1
}

main() {
  local input="$1"

  if [[ $input == "setup" ]]; then
    setup
  elif [[ $input == "cmd" ]]; then
    cmd
  elif [[ $input == "assert" ]]; then
    assert
  elif [[ $input == "name" ]]; then
    name
  else
    exit 1
  fi
}
