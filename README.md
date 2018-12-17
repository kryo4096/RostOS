# RostOS
RostOS is an operating system written in Rust, a relatively new systems programming language. Its roots lie in Phillip Oppermanns excellent tutorial series ["Writing an OS in Rust"](https://os.phil-opp.com/), which helped me grasp the fundamentals of low-level programming and served as a perfect starting point for the Project. From there on new features were added to reach its current state.

Right now the Kernel's features include the following:
* It boots on many systems running the x86_64 architecture.
* It can display text using the legacy VGA buffer.
* It reads PS/2 keyboard input.
* It can start userspace applications from elf-files on a ramdisk.
* The ramdisk has its own format *RostFS* to save files.
* Multiple processes can be ran at the same time using a simple scheduler.

To make use of these features, there are system calls which are abstracted a way in a small library. Some example programs have been written, including a simple console (TODO) and Pong.

## Installation and Testing

*DISCLAIMER: To build RostOS, Linux is recommended. Building on windows or macOS should technically be possible, but has not been tested and no automatic build scripts are provided.* 

To begin, you first need to download the language itself. Because an OS requires certain unstable features, a nightly version of the Rust toolchain has to be installed. This can be done using [rustup](https://rustup.rs/), the official rust toolchain installation program. You also have to add the `.cargo/bin` directory to `$PATH` so you can use the buildtools installed later.

If you want to run the OS you need some sort of virtual machine. To use the automatic build scripts, the `qemu-system-x86_64` binary has to be present in `$PATH?, which can be downloaded from [www.qemu.org](https://www.gemu.org) or preferably installed with your distribution's package manager.

When these prerequisites are met, you can run `build_tools.sh` inside the `RostOS` directory, which will install the required build tools using `cargo install` and `rustup`. These are:

* `fscreate` to create a ramdisk image from a template folder.
* `bootimage` to append the kernel to the bootloader
* `cargo xbuild` to cross-compile to custom targets

The script will also install some dependencies with rustup and set the default toolchain for the `RostOS` folder to nightly. 
Once the install script is done, you can compile and run the kernel. Use `run_vtx.sh` if you have intel virtualization (VT-x) and otherwise `run.sh` for software emulation. 

To build a bootable image, use `build.sh`, which will create a binary at `bin/RostOS.bin`. 






