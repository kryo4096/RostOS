#include "syscall.h"
#include "std.h"
#include "path.h"

// if this breaks, it's probably because the calling covention changed
uint64_t syscall(uint64_t rdi, uint64_t rsi, uint64_t rdx, uint64_t rcx, uint64_t r8, uint64_t r9) {
    asm("int $0x80");
}


// old syscall functions

/*
void print(char* str) {

    size_t len = strlen(str);

    asm("mov $0, %%rdi" ::: "rdi"); 
    asm("mov %0, %%rsi" :: "r" (str) : "rsi");
    asm("mov %0, %%rdx" :: "r" (len): "rdx");
    asm("int $0x80":::"rax");       
}

void debug_num(uint64_t num, format f) {

    syscall(2,num, f, 0, 0, 0);

    //asm("mov $2, %%rdi" ::: "rdi"); 
    //asm("mov %0, %%rsi" :: "r" (num) : "rsi");
    //asm("mov %0, %%rdx" :: "r" ((uint64_t)f): "rdx");
    //asm("int $0x80":::"rax");       
}

uint64_t get_ticks() {
    uint64_t ticks;
    asm("mov $0x10, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (ticks)::);
    return ticks;
}

uint8_t get_scancode() {
    uint64_t scancode;
    asm("mov $0x20, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (scancode)::);
    return (uint8_t) scancode;
}

uint64_t get_pid() {
    uint64_t pid;
    asm("mov $0x30, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (pid)::);
    return pid;
}

void sys_exit() {
    asm("mov $0x31, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
}

uint64_t execute(path_t path) {
    uint64_t pid;
    size_t len = strlen(path);
    asm("mov %0, %%rsi" :: "r" (path) : "rsi");
    asm("mov %0, %%rdx" :: "r" (len): "rdx");
    asm("mov $0x32, %%rdi" ::: "rdi");
    asm("int $0x80" ::: "rax");
    asm("mov %%rax, %0": "=r" (pid)::);

    return pid;
}

void wait_pid(uint64_t pid) {
    asm("mov %0, %%rsi" :: "r" (pid) : "rsi");
    asm("mov $0x33, %%rdi" ::: "rdi");
    asm("int $0x80" ::: "rax");
}

uint64_t map_vga() {
    uint64_t addr;
    asm("mov $0x40, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (addr)::);
    return addr;
}

uint64_t unmap_vga() {
    uint64_t addr;
    asm("mov $0x41, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (addr)::);
    return addr;
}

