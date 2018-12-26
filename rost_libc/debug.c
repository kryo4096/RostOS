#include "syscall.h"
#include "debug.h"


void debug_print(char * str) {
    syscall(SYS_DEBUG_PRINT, str, strlen(str), 0, 0, 0);
}

void debug_printnum(uint64_t number, debugf_t format) {
    syscall(SYS_DEBUG_PRINTNUM, number, (uint64_t) format, 0, 0, 0);
}

