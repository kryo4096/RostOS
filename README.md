# RostOS
RostOS is an operating system written in Rust, a relatively new systems programming language. Its roots lie in Phillip Oppermanns excellent tutorial series ["Writing an OS in Rust"](https://os.phil-opp.com/), which helped me grasp the fundamentals of low-level programming and served as a perfect starting point for the Project. From there on new features were added to reach its current state.

Right now the Kernel's features include the following:
* It boots on many systems running the x86_64 architecture.
* It can display text using the legacy VGA buffer.
* It reads PS/2 keyboard input.
* It can start userspace applications from ELF-files on a ramdisk.
* The ramdisk has its own format *RostFS* to save files.
* Multiple processes can be run at the same time using a simple scheduler.

To make use of these features, there are system calls which are abstracted away in a small library. Some example programs have been written, including a simple console (TODO) and Pong.

## Installation and Testing

*DISCLAIMER: To build RostOS, Linux is recommended. Building on windows or macOS should technically be possible, but has not been tested and no automatic build scripts are provided.* 

To begin, you first need to download the language itself. Because an OS requires certain unstable features, a nightly version of the Rust toolchain has to be installed. This can be done using [rustup](https://rustup.rs/), the official rust toolchain installation program. You also have to add the `.cargo/bin` directory to `$PATH` so you can use the buildtools installed later.

Right now, RostOS officially only supports running C programs compiled to ELF. To build these a C compiler is required, per default `gcc`. If you wish to use a different one, you can change the line containing `gcc` in `build.sh` and change it accordingly.

If you want to run the OS you need some sort of virtual machine. To use the automatic build scripts, the `qemu-system-x86_64` binary has to be present in `$PATH`, which can be downloaded from [www.qemu.org](https://www.qemu.org) or preferably installed with your distribution's package manager.

When these prerequisites are met, you can run `install_tools.sh` inside the `RostOS` directory, which will install the required build tools using `cargo install` and `rustup`. These are:

* `fscreate` to create a ramdisk image from a template folder.
* `bootimage` to append the kernel to the bootloader
* `cargo xbuild` to cross-compile to custom targets

The script will also install some dependencies with rustup and set the default toolchain for the `RostOS` folder to nightly. 
Once the install script is done, you can compile and run the kernel. Use `run_vtx.sh` if you have intel virtualization (VT-x) and otherwise `run.sh` for software emulation. 

If everything works, a shell should appear. You can type in the name of a program and it should run. To see which programs are available, you can take a look at the `programs` folder. (If you're too lazy try `pong`.)

To build a bootable image to run on a real machine, use `build.sh`, which will create a binary at `bin/RostOS.bin`. 

## Creating your own program

If the underwhelming amount of available programs bores you, you can create your own without too much effort. Just run `create_program.sh` with a name as an argument. A folder will appear at `programs/{name}` in which you can write your program. The standard library is not documented yet, but still quite manageable. To help get you started, an example `main.c` is provided. You could also take look at the existing programs' sources, which contain comments and are quite readable.

Running your custom program is easy: `build.sh` will automatically compile all program folders and link them against `rost_std`, whose headers are automatically included. `run.sh` and `run_vtx.sh` use that script and therefore work the same way. If your program fails to compile, the kernel will still be started but your program's binary will not be present.

### Guidelines

* No heap is available, so you cannot use `malloc`.
* Officially, only the `rost_std` headers can be used.
* Try not to use `syscall.h` but the the other headers, which are mostly abstractions around it. (`syscall.h` is very hacky right now anyways)






