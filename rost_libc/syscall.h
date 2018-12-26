#ifndef SYSCALL_H
#define SYSCALL_H
#include "std.h"

#define SYS_DEBUG_PRINT 0x0
#define SYS_DEBUG_PRINTNUM 0x2

#define SYS_TIME_GETTICKCOUNT 0x10

#define SYS_KB_POPSCANCODE 0x20

#define SYS_PROC_GETPID 0x30
#define SYS_PROC_EXIT 0x31
#define SYS_PROC_EXECUTE 0x32
#define SYS_PROC_WAITPID 0x33

#define SYS_VGA_MAP 0x40
#define SYS_VGA_UNMAP 0x41




uint64_t syscall(uint64_t rdi, uint64_t rsi, uint64_t rdx, uint64_t rcx, uint64_t r8, uint64_t r9);


#endif