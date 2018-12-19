#ifndef PROCESS_H
#define PROCESS_H

#include "std.h"
#include "path.h"
#include "syscall.h"

typedef uint64_t pid_t;

pid_t proc_spawn(path_t elf_path);

void proc_wait(pid_t pid);

void proc_exit();

#endif