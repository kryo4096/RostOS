#!/bin/bash
./build.sh
qemu-system-x86_64 bin/RostOS.bin -enable-kvm -machine q35,accel=kvm -device intel-iommu \
    -cpu host -m 2G


