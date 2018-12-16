#!/bin/bash

mkdir -p ramdisk/bin

for dir in rost_programs/*; do
    cd ${dir}
    cargo xbuild --target ../../x86_64-rost_os.json
    cp target/x86_64-rost_os/debug/$(basename ${dir}) ../../ramdisk/bin/$(basename ${dir})
    cd ../..

    
done

fscreate rost_kernel/disk.img 1024 ramdisk

cd rost_kernel

bootimage run -- -enable-kvm -machine q35,accel=kvm -device intel-iommu \
    -cpu host -d int -m 2G -monitor stdio


#bootimage build
objdump -d target/x86_64-rust_kernel/debug/rost_kernel -M intel > kernel.dmp


#qemu-system-x86_64 target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin \


