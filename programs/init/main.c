#include "syscall.h"
#include "std.h"
#include "keyboard.h"

void _start() {
    uint64_t pid = execute("/bin/shell");
    wait_pid(pid);
    println("Shell exited!");

    proc_exit();
}
