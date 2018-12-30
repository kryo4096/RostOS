#!/bin/bash

rm -rf ramdisk/bin
mkdir -p ramdisk/bin

for dir in programs/*; do
    if [[ -f $dir/Cargo.toml ]]; then
        old_path=$(pwd)
        cd $dir
        cargo xbuild --release --target $old_path/x86_64-rost_os.json 
        cd $old_path
        cp $dir/target/x86_64-rost_os/release/$(basename $dir) $old_path/ramdisk/bin/
    else
        gcc -Irost_libc -mno-red-zone -nostartfiles $dir/* rost_libc/* -o ramdisk/bin/$(basename $dir) || rm ramdisk/bin/$(basename $dir)
    fi
done

fscreate disk.img 1024 ramdisk

cd rost_kernel

bootimage build

mkdir ../bin
rm ../bin/*.bin
cp target/x86_64-rust_kernel/debug/bootimage-rost_kernel.bin ../bin/RostOS.bin


#bootimage build
objdump -d target/x86_64-rust_kernel/debug/rost_kernel -M intel > kernel.dmp


#qemu-system-x86_64 target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin \


