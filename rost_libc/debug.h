#ifndef DEBUG_H
#define DEBUG_H
#include "std.h"


typedef enum debugf {
    BINARY = 0x0,
    OCTAL = 0x1,
    DECIMAL = 0x2,
    HEXADECIMAL = 0x3,
} debugf_t;

void debug_print(char * str);
void debug_printnum(uint64_t number, debugf_t format);

#endif