#include "std.h"
#include "process.h"
#include "terminal.h"
#include "keyboard.h"

void _start() {
    asm("mov $0, %%cr3");
}