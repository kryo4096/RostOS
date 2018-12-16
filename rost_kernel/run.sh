#!/bin/bash
rm root/bin/*

for dir in programs/*; do
    gcc -Ilibc -mno-red-zone -nostartfiles ${dir}/* libc/* -o root/bin/$(basename ${dir})
done

fscreate disk.img 1024 root

bootimage run -- -enable-kvm -machine q35,accel=kvm -device intel-iommu \
    -cpu host -d int -m 2G -monitor stdio


#bootimage build
objdump -d target/x86_64-rust_kernel/debug/rust_kernel -M intel > kernel.dmp


#qemu-system-x86_64 target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin \


