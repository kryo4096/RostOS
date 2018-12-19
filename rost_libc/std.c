#include "std.h"
#include "syscall.h"

size_t strlen(char* str) {
    size_t i = 0;

    while(str[i]) {
        i++;
    }

    return i;
}

void halt() {
    asm("hlt");
}

void wait(uint64_t tick_count) {
    uint64_t until = get_ticks() + tick_count;

    while (get_ticks() < until);
}