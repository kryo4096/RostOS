#ifndef STD_H
#define STD_H
#include "stdint.h"
#include "stddef.h"
#include "stdbool.h"

size_t strlen(char* str);

void halt();

void wait(uint64_t tick_count);

#endif