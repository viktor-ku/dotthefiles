#!/bin/bash

source ../../lib.sh

setup() {
  mkdir dist
}

cmd() {
  echo "dtf ln dtf.yml --os macos"
}

assert() {
  test -f dist/file.txt
}

main "$1"
