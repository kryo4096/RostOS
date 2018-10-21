#!/bin/bash

cargo xbuild --target x86_64-bootloader --release
objcopy -O binary -S target/x86_64-bootloader/release/bootloader bootimage.bin
qemu-system-x86_64 -hda bootimage.bin -d int -s