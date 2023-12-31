#!/bin/bash

# This script is for integration tests of unreleased versions of rucksack
# that are still in-development.

make build

. ./tests/common.sh || . ./common.sh

echo
header "Show top-level help"
./$BIN --help
echo

header "View entire parsed data set"
./$BIN --input "$MD_FILE" --pretty
echo

header "View filtered subset"
./$BIN --input "$MD_FILE" --pretty \
--query '.children.nodes[] | select(((.depth == 3) and .name == "heading") and .source == "Getting Started")'
echo

header "View HTML for all headings of depth 3"
./$BIN --input "$MD_FILE" \
--query '.children.nodes[] | select((.depth == 3) and .name == "heading") | .children.nodes[].html'
echo

header "View the parsed front matter for the Markdowwn file"
./$BIN --input "$MD_FILE" \
--query '.children.nodes[] | select(.name == "toml") | .json' | \
jq -r . | jq .frontmatter
echo

header "Process markdown to file ($TMP_DIR/out.json)"
mkdir -p $TMP_DIR
./$BIN --input "$MD_FILE" --output "$TMP_DIR/out.json"
echo

header "Read from $TMP_DIR/out.json"
cat "$TMP_DIR/out.json"
echo
echo

header "Process again to same markdown file ($TMP_DIR/out.json)"
./$BIN --input "$MD_FILE" --output "$TMP_DIR/out.json"
echo

header "Re-read from $TMP_DIR/out.json"
cat "$TMP_DIR/out.json"
echo
echo

header "Skip processing, read from $TMP_DIR/out.json, and apply query"
./$BIN --skip-process --input "$TMP_DIR/out.json" \
--query '.children.nodes[] | select((.depth == 3) and .name == "heading") | .children.nodes[].source'
echo
