#include "syscall.h"
#include "std.h"
#include "keyboard.h"

void _start() {
    println("Init started!");

    uint64_t pid = execute("/bin/pong");
    wait_pid(pid);
    println("Pong exited!");

    proc_exit();
}
