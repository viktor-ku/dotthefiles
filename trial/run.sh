#!/bin/bash

cases_dir=trial/cases
pwd=$(pwd)

export PATH=$pwd/target/debug:$PATH

commit=$(git log -1 --pretty="%h")
date=$(date +"%I:%M%p %B%e, %G")

echo "# Trial \`$(git log -1 --pretty="%h")\`"
echo
echo "- \`$commit\`"
echo "- $date"

run() {
  local one=$1

  echo
  echo "### Testing $one"

  local errno
  local base_dir="$pwd/$cases_dir/$one"
  local test=./test.sh

  cd "$base_dir"

  local name=$($test name)
  test -n "$name" && echo "> $name"

  $test setup
  errno=$?
  if [[ $errno == "0" ]]; then
    echo "- setup done"
  else
    echo "- setup failed"
    return $errno
  fi

  local cmd=$($test cmd)
  echo "- running \`$cmd\`"

  $cmd
  errno=$?
  if [[ $errno != "0" ]]; then
    echo "- cmd itself exited with non-zero ($errno)"
    return "$errno"
  fi

  $test assert
  errno=$?
  if [[ $errno == "0" ]]; then
    echo "- assert OK"
  else
    echo "- assert FAILED"
    return $errno
  fi
}

compute_status() {
  local n_total=$1
  local n_passed=$2
  local n_failed=$3

  if [[ $n_failed == 0 ]]; then
    echo "OK"
  else
    echo "FAILED"
  fi
}

main() {
  local list=$1

  local n_total=0
  local n_passed=0
  local n_failed=0

  for one in $list; do
    n_total=$((n_total + 1))
    run "$one"
    local errno=$?
    if [[ $errno == "0" ]]; then
      n_passed=$((n_passed + 1))
    else
      n_failed=$((n_failed + 1))
    fi
  done

  local status=$(compute_status $n_total $n_passed $n_failed)

  echo
  echo "## Summary"
  echo "\`\`\`"
  echo "test result: $status. $n_total total; $n_passed passed; $n_failed failed"
  echo "\`\`\`"
}

main "$(ls $cases_dir)"
