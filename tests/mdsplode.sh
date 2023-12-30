#!/bin/bash

# This script is for integration tests of latest released version of mdsplode.

. ./tests/common.sh || . ./common.sh

rm -f ~/.cargo/$BIN ~/.cargo/registry/cache/github.com*/mdsplode*

echo
header "Install mdsplode"
cargo install mdsplode

echo
header "Show top-level help"
mdsplode --help
echo

header "View entire parsed data set"
mdsplode --input "$MD_FILE" --pretty
echo

header "View filtered subset"
mdsplode --input "$MD_FILE" --pretty \
--query '.children.nodes[] | select(((.depth == 3) and .name == "heading") and .source == "Getting Started")'
echo

header "View HTML for all headings of depth 3"
mdsplode --input "$MD_FILE" \
--query '.children.nodes[] | select((.depth == 3) and .name == "heading") | .children.nodes[].html'
echo

header "View the parsed front matter for the Markdowwn file"
mdsplode --input "$MD_FILE" \
--query '.children.nodes[] | select(.name == "toml") | .json' | \
jq -r . | jq .frontmatter
echo
