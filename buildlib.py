
import subprocess as sp
import shutil as su
import os

QEMU_CMD = ["qemu-system-x86_64"]
QEMU_ARGS = ["-m", "2G"]
QEMU_VT_ARGS =  ["-enable-kvm", "-machine", "q35,accel=kvm", "-device", "intel-iommu"] 


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

def build():

    if su.which("rustup") == None:
        print("Rust is not installed for the current user. Go to https://rustup.rs and follow the instructions, then rerun the script!")
        exit(-1)
        return None

    sp.call("rustup toolchain install nightly")
    sp.call("rustup override add nightly") 
    sp.call("rustup component add rust-src")


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


def run(virtualize):
    if su.which(QEMU_CMD[0]) == None:
        print("QEMU not installed, get it on https://www.qemu.org or from your distribution's package manager!")
        return None

    img = build()

    if img == None:
        print("Failed to compile RostOS!")
        return None
    
    print("Running RostOS!")

    if virtualize:
        result = sp.call(QEMU_CMD + [img] + QEMU_ARGS + QEMU_VT_ARGS)
        if result != 0:
            print("Virtualization using VT-x is not supported on this computer. Try rerunning the script without the -vtx argument")
            return None
        else:
            return 0
    else:
        sp.call(QEMU_CMD + [img] + QEMU_ARGS)
        return 0
