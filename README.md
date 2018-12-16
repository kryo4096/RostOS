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

*DISCLAIMER: To build RostOS Linux is recommended. Building on windows or macOS should technically be possible, but has not been tested and no automatic build scripts are provided.* 

A nightly version of the Rust toolchain has to be installed, which is best done using [rustup](https://rustup.rs/)). The `~/.cargo/bin` folder has to be added to `$PATH`

Additionally the `qemu-system-x86_64` binary has to be present in `$PATH?, which can be downloaded from [www.qemu.org](https://www.gemu.org) or preferably installed with your distribution's package manager.









