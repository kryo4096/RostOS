#ifndef SYSCALL_H
#define SYSCALL_H
#include "std.h"

void println(char* str);

void print(char* str);

typedef enum format {
    BINARY = 0x0,
    OCTAL = 0x1,
    DECIMAL = 0x2,
    HEXADECIMAL = 0x3,
} format;

void debug_num(uint64_t num, format f);

uint64_t get_ticks();
uint8_t get_scancode();


uint64_t get_pid();

uint64_t map_vga();
uint64_t unmap_vga();

void sys_exit();
uint64_t execute(char* path);
void wait_pid(uint64_t pid);

#endif