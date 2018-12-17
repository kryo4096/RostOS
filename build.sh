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

bootimage build

mkdir ../bin
rm ../bin/*.bin
cp target/x86_64-rust_kernel/debug/bootimage-rost_kernel.bin ../bin/RostOS.bin


#bootimage build
objdump -d target/x86_64-rust_kernel/debug/rost_kernel -M intel > kernel.dmp


#qemu-system-x86_64 target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin \


