#!/bin/sh

dtf=target/debug/dtf

all=$(ls tests)

echo

for one in $all; do
  if [ $one == "run.sh" ]; then
    continue;
  fi

  mkdir tests/$one/dist
  $dtf ln tests/$one/dtf.yml >> /dev/null

  ./tests/$one/test.sh $one
  res=$?

  if [ $res == "1" ]; then
    echo - testing $one... FAILED
  else
    echo + testing $one... OK
  fi
done

echo
echo Done
