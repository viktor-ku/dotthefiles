#!/bin/bash

source ../../lib.sh

setup() {
  mkdir dist
}

cmd() {
  echo "dtf ln dtf.json --os macos"
}

assert() {
  test -f dist/file.txt
}

name() {
  echo ".json config file should also work"
}

main "$1"
