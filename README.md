# RostOS
RostOS is an operating system written in Rust, a relatively new systems programming language. Phillip Oppermanns excellent tutorial series ["Writing an OS in Rust"](https://os.phil-opp.com/) helped me grasp the fundamentals of low-level programming and served as a perfect starting point for the Project. From there on many features were added to reach its current state.

Right now the Kernel's features include the following:
* It boots on many systems running the x86_64 architecture.
* It can display text using the legacy VGA buffer.
* It reads PS/2 keyboard input.
* It can start userspace applications from ELF-files on a ramdisk.
* The ramdisk has its own format *RostFS* to save files.
* Multiple processes can be run at the same time using a simple scheduler.

To make use of these features, there are system calls which are abstracted away in a small library. Some example programs have been written, including a simple shell and pong.

## Installation and Testing

To begin, you first need to download the language itself. Because an OS requires certain unstable features, a nightly version of the Rust toolchain has to be installed. This can be done using [rustup](https://rustup.rs/), the official rust toolchain installation program. 

To run the OS you need some sort of virtual machine. To use the automatic build scripts, the `qemu-system-x86_64` binary has to be present in `$PATH`, which can be downloaded from [www.qemu.org](https://www.qemu.org) or preferably installed with your distribution's package manager.

Using the automatic build scripts below also requires Python 3 to be installed.

Once these prerequisites are available, you can automatically install all additional dependencies by running `python rost.py install`. If the install script works, you can then build the OS using `python rost.py build` and then `python rost.py run` to run it.

The build command will automatically build any rust projects found in the `programs` folder and include them in the OS' `bin` directory.

To write your own programs you can take a look at the `pong` program. If you copy it, delete its logic and change its name in `Cargo.toml` you'll have a nice template.



