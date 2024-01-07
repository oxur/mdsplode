#!/bin/bash

#make build

. ./tests/common.sh || . ./common.sh

FIFO=testfifo

echo
header "Show mdsplode session"
rm -f $FIFO
mkfifo $FIFO
tail -f $FIFO | ./$BIN shell --headless &
echo banner > $FIFO
sleep 1
echo ping > $FIFO
sleep 1
echo version > $FIFO
sleep 1
echo help > $FIFO
sleep 1
echo help show > $FIFO
sleep 1
echo history > $FIFO
sleep 1
echo quit > $FIFO
sleep 1
ps aux | grep 'mdsplode shell' | grep -v grep | awk '{print $2}' | xargs kill -9
rm $FIFO
echo
