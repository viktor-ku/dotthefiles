#!/bin/bash

source ../../lib.sh

setup() {
  mkdir dist
}

cmd() {
  echo "dtf ln dtf.yml"
}

assert() {
  test -f dist/file.txt
}

main "$1"
