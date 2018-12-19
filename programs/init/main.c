#include "syscall.h"
#include "std.h"
#include "keyboard.h"
#include "process.h"

void _start() {
    uint64_t pid = proc_spawn("/bin/shell");
    proc_wait(pid);
    proc_exit();
}
