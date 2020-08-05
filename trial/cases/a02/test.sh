#!/bin/bash

source ../../lib.sh

setup() {
  mkdir dist
  echo another > dist/file.txt
}

cmd() {
  echo "dtf ln dtf.yml -f --os macos"
}

assert() {
  test -f dist/file.txt
  test dist/file.txt -ef files/macos/file.txt
}

name() {
  echo "it should replace already existing destination file by using -f option"
}

main "$1"
