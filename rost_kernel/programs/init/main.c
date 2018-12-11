#include<stddef.h>
#include<stdint.h>

size_t strlen(char* str) {
    size_t i = 0;

    while(str[i]) {
        i++;
    }

    return i;
}

void println(char* str) {

    size_t len = strlen(str);

    asm("mov $1, %%rdi" ::: "rdi"); 
    asm("mov %0, %%rsi" :: "r" (str) : "rsi");
    asm("mov %0, %%rdx" :: "r" (len): "rdx");
    asm("int $0x80");       
}

void print(char* str) {

    size_t len = strlen(str);

    asm("mov $0, %%rdi" ::: "rdi"); 
    asm("mov %0, %%rsi" :: "r" (str) : "rsi");
    asm("mov %0, %%rdx" :: "r" (len): "rdx");
    asm("int $0x80":::"rax");       
}

typedef enum format {
    BINARY = 0x0,
    OCTAL = 0x1,
    DECIMAL = 0x2,
    HEXADECIMAL = 0x3,
} format;

void debug_num(uint64_t num, format f) {

    asm("mov $2, %%rdi" ::: "rdi"); 
    asm("mov %0, %%rsi" :: "r" (num) : "rsi");
    asm("mov %0, %%rdx" :: "r" ((uint64_t)f): "rdx");
    asm("int $0x80":::"rax");       
}

uint64_t get_ticks() {
    uint64_t ticks;
    asm("mov $0x10, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (ticks)::);
    return ticks;
}

uint8_t get_scancode() {
    uint8_t scancode;
    asm("mov $0x20, %%rdi" ::: "rdi"); 
    asm("int $0x80" :::"rax");
    asm("mov %%rax, %0": "=r" (scancode)::);
    return scancode;
}

void wait(uint64_t tick_count) {
    uint64_t ticks = get_ticks();

    while ((get_ticks() - ticks) < tick_count);
}

void _start() {
    while(1) {
        uint8_t scancode = get_scancode();
        if(scancode!=0) {
            debug_num(scancode)
        }
    }
}