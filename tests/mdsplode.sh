#!/bin/bash

# This script is for integration tests of latest released version of rucksack.

. ./tests/common.sh || . ./common.sh

rm -f ~/.cargo/$BIN ~/.cargo/registry/cache/github.com*/rucksack*

echo
header "Install mdsplode"
echo

cargo install mdsplode
