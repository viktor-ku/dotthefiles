#!/bin/sh

dir=$1

test -f tests/$dir/dist/file.txt
exit $?
