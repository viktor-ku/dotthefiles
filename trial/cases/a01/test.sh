#!/bin/bash

setup() {
  mkdir dist
}

cmd() {
  echo "dtf ln dtf.yml"
}

assert() {
  test -f dist/file.txt
}

main() {
  local input="$1"

  if [[ $input == "setup" ]]; then
    setup
  elif [[ $input == "cmd" ]]; then
    cmd
  elif [[ $input == "assert" ]]; then
    assert
  else
    exit 1
  fi
}

main "$1"
