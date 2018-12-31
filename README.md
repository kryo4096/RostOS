# RostOS
RostOS is an operating system written in Rust, a relatively new systems programming language. Its roots lie in Phillip Oppermanns excellent tutorial series ["Writing an OS in Rust"](https://os.phil-opp.com/), which helped me grasp the fundamentals of low-level programming and served as a perfect starting point for the Project. From there on new features were added to reach its current state.

Right now the Kernel's features include the following:
* It boots on many systems running the x86_64 architecture.
* It can display text using the legacy VGA buffer.
* It reads PS/2 keyboard input.
* It can start userspace applications from ELF-files on a ramdisk.
* The ramdisk has its own format *RostFS* to save files.
* Multiple processes can be run at the same time using a simple scheduler.

To make use of these features, there are system calls which are abstracted away in a small library. Some example programs have been written, including a simple shell and Pong.

## Installation and Testing

To begin, you first need to download the language itself. Because an OS requires certain unstable features, a nightly version of the Rust toolchain has to be installed. This can be done using [rustup](https://rustup.rs/), the official rust toolchain installation program. You also have to add the `~/.cargo/bin` directory to `$PATH` so you can use the buildtools installed later.

Right now, RostOS officially only supports running C programs compiled to ELF. To build these a C compiler is required, per default `gcc`. If you wish to use a different one, you can change the line containing `gcc` in `build.sh` accordingly.

If you want to run the OS you need some sort of virtual machine. To use the automatic build scripts, the `qemu-system-x86_64` binary has to be present in `$PATH`, which can be downloaded from [www.qemu.org](https://www.qemu.org) or preferably installed with your distribution's package manager.

Once these prerequisites are available, you can run the kernel using `run.py`.

If everything works, a shell should appear. You can type in the name of a program and it should run. To see which programs are available, you can take a look at the `programs` folder. (If you're too lazy try `pong`.)

To build a bootable image to run on a real machine, use `build.sh`, which will create a binary at `bin/RostOS.bin`. 







