#!/bin/bash

setup() {
  mkdir dist
  echo another > dist/file.txt
}

cmd() {
  echo "dtf ln dtf.yml -f"
}

assert() {
  test -f dist/file.txt
  test dist/file.txt -ef files/macos/file.txt
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
