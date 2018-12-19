#include "std.h"
#include "process.h"
#include "terminal.h"
#include "keyboard.h"

void _start() {
    // Initialize a new virtual terminal. There is no stdin / stdout yet.
    vt_init();
    // Print to that terminal (not to shell)
    vt_println("Hello, RostOS!");

    // Wait until the "any key" is pressed. 
    kb_wait_any();

    // Exit the process. Never forget this.
    proc_exit();
}