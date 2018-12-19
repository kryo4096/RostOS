#include "process.h"

pid_t proc_spawn(path_t elf_path) {
    return execute(elf_path);
}

void proc_wait(pid_t pid) {
    wait_pid(pid);
}

void proc_exit() {
    sys_exit();
}