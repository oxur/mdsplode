BIN=bin/mdsplode
export RUST_BACKTRACE=1
TMP_DIR=/tmp/$(date +"%Y%m%d.%H%M%S")/mdsplode
MD_FILE=./tests/data/learn.md
MD_DIR=./tests/data/multi

GREEN="\033[0;32m"
YELLOW="\033[1;33m"
END_COLOR="\033[0m"

function header () {
    echo -e "${GREEN}>>${END_COLOR} ${YELLOW}${1}${END_COLOR}"
    echo
}
