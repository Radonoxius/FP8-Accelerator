#!/bin/bash

DE_HOME='sdcard/home/root'
ME=$(whoami)

RUST_BUILD_DIR='target/armv7-unknown-linux-musleabihf'
DISK='sdc2'

#sudo umount /media/$ME/e3235a93-c4b0-4371-8e13-8b492281ed57
mkdir -p sdcard
sudo mount -o rw /dev/$DISK sdcard

sudo rm $DE_HOME/soft-fp8
sudo cp $RUST_BUILD_DIR/release/soft-fp8 $DE_HOME/

sync
sudo umount sdcard