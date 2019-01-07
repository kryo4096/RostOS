#!/usr/bin/env python
import subprocess as sp
import shutil as su
import os
import sys

QEMU_CMD = ["qemu-system-x86_64"]
QEMU_ARGS = ["-m", "2G", "-monitor", "stdio"]
QEMU_VT_ARGS_LINUX =  ["-enable-kvm", "-machine", "q35,accel=kvm", "-device", "intel-iommu"] 
QEMU_VT_ARGS_WINDOWS =  ["-enable-kvm", "-machine", "q35,accel=hax"] 

CARGO_DIR = os.path.expanduser("~") + "/.cargo/bin"

TARGET_NAME = "x86_64-rost_os"
TARGET_JSON = os.getcwd() + "/" + TARGET_NAME + ".json"

RAMDISK_SIZE = "1024" # 1MB
RAMDISK_PATH = "disk.img"
RAMDISK_SRC = "ramdisk"

BINARY_DIR = "bin"

KERNEL_DIR = "rost_kernel"
KERNEL_BIN = KERNEL_DIR + "/target/x86_64-rust_kernel/debug/bootimage-rost_kernel.bin"

ROSTOS_BIN = "bin/RostOS.bin"

def install():
    if su.which("rustup") == None:
        print("Rust is not installed for the current user. Go to https://rustup.rs and follow the instructions, then rerun the script!")
        exit(-1)
        return None

    sp.call(["rustup", "toolchain", "install", "nightly"])
    sp.call(["rustup", "override", "add", "nightly"]) 
    sp.call(["rustup", "component", "add", "rust-src"])


    if sp.call(["cargo", "install", "--force", "--path", "rost_fs/fscreate"]) == 0:
        if su.which("fscreate") == None:
            os.environ["PATH"] += os.pathsep + CARGO_DIR
    else:
        print("failed to compile fscreate")
        return None

    if su.which("bootimage") == None:
        if sp.call(["cargo", "install", "bootimage"]) != 0:
            print("failed to compile bootimage")
            return None

    if su.which("cargo-xbuild") == None:
        if sp.call(["cargo", "install", "cargo-xbuild"]) != 0:
            print("failed to compile cargo xbuild")
            return None

def build():
    if not os.path.isdir("ramdisk"):
        os.mkdir("ramdisk")
        os.mkdir("ramdisk/bin")
    elif not os.path.isdir("ramdisk/bin"):
        os.mkdir("ramdisk/bin")

    for program in os.listdir("programs"):
        program_path = "programs/" + program
        
        compilation_result = sp.call(["cargo", "xbuild", "--target", TARGET_JSON], cwd=program_path)
        if compilation_result == 0:
            su.copy2(program_path + "/target/" + TARGET_NAME + "/debug/" + program, RAMDISK_SRC + "/bin/" + program)
            print("COMPILATION SUCCEEDED! (" + program_path + ")")
        else:
            print("COMPILATION FAILED! (" + program_path + ")")
            return None

    sp.call(["fscreate", RAMDISK_PATH, RAMDISK_SIZE, RAMDISK_SRC])

    if not os.path.isdir("bin"):
        os.mkdir("bin")

    compilation_result = sp.call(["bootimage", "build"], cwd=KERNEL_DIR)

    if compilation_result == 0:
        su.copy2(KERNEL_BIN, ROSTOS_BIN)
        return ROSTOS_BIN
    else:
        return None


def run(img, virtualize):
    if su.which(QEMU_CMD[0]) == None:
        print("QEMU not installed, get it on https://www.qemu.org or from your distribution's package manager!")
        return None
        
    if img == None:
        print("Failed to compile RostOS!")
        return None
    
    print("Running RostOS!")

    if virtualize:
        if sp.call(QEMU_CMD + [img] + QEMU_ARGS + QEMU_VT_ARGS_LINUX) == 0:
            return 0
        if sp.call(QEMU_CMD + [img] + QEMU_ARGS + QEMU_VT_ARGS_WINDOWS) == 0:
            return 0
            
        print("Virtualization using VT-x is not supported on this computer.")
        return None

    else:
        sp.call(QEMU_CMD + [img] + QEMU_ARGS)
        return 0


if len(sys.argv) < 2:
    print("Usage: rost.py [option]")
    print("install - installs some tools required by RostOS")
    print("build - builds RostOS")
    print("run - runs RostOS, doesn't build it")
    print("build_run - builds, then runs RostOS")
    exit(-1)

option = sys.argv[1]

if option == "install":
    install()

if option == "build":
    build()

if option == "run":
    run(ROSTOS_BIN, False)

if option == "build_run":
    if build():
        run(ROSTOS_BIN, False)

if option == "virt":
    run(ROSTOS_BIN, True)

