#!/bin/bash
rm root/bin/*

for dir in programs/*; do
    gcc -nostartfiles ${dir}/main.c -o root/bin/$(basename ${dir})
done

fscreate disk.img 512 root

bootimage run