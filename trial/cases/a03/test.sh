#!/bin/bash

source ../../lib.sh

setup() {
  mkdir dist
}

cmd() {
  echo "dtf ln dtf.yml --os arch"
}

assert() {
  test -f dist/file.txt && exit 1 || exit 0
}

name() {
  echo "it should not link because target is set to macos, but dtf is run like it is arch"
}

main "$1"
