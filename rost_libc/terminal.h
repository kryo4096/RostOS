#ifndef TERMINAL_H
#define TERMINAL_H
#include "std.h"

void vt_init();

void vt_putchar(char c);
void vt_delchar();
void vt_newln();

void vt_print(char *string);
void vt_println(char *string);


void vt_clear();
void vt_set_color(uint8_t color_code);

char* vt_readln();

#endif