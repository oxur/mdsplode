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

header "Process markdown to file ($TMP_DIR/out.json)"
mkdir -p $TMP_DIR
mdsplode --input "$MD_FILE" --output "$TMP_DIR/out.json"
echo

header "Read from $TMP_DIR/out.json"
cat "$TMP_DIR/out.json"
echo
echo

header "Process again to same markdown file ($TMP_DIR/out.json)"
mdsplode --input "$MD_FILE" --output "$TMP_DIR/out.json"
echo

header "Re-read from $TMP_DIR/out.json"
cat "$TMP_DIR/out.json"
echo
echo

header "Skip processing, read from $TMP_DIR/out.json, and apply query"
mdsplode --skip-process --input "$TMP_DIR/out.json" \
--query '.children.nodes[] | select((.depth == 3) and .name == "heading") | .children.nodes[].source'
echo

header "Process multiple markdown files ($MD_DIR/)"
rm -rf $TMP_DIR/many-md-files
mkdir -p $TMP_DIR/many-md-files
mdsplode --input "$MD_DIR" --output "$TMP_DIR/many-md-files"
find $TMP_DIR/many-md-files -type f -ls
echo

header "View title from each of the multiple files"
mdsplode --skip-process --input "$TMP_DIR/many-md-files" --log-level info \
--query '.children.nodes[] | select(.name == "toml") | .json' | \
tr ',' '\n'|egrep -e '"title|INFO'
echo
