#!/bin/bash

source ../../lib.sh

setup() {
  mkdir dist
}

cmd() {
  echo "dtf ln dtf.toml"
}

assert() {
  test -f dist/file.txt
}

name() {
  echo ".toml config file should also work"
}

main "$1"
