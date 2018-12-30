#include "syscall.h"
#include "std.h"
#include "keyboard.h"
#include "process.h"
static uint64_t PID;

void _start() {

    PID = get_pid();
    uint64_t ticks = get_ticks();
    while(1) {

        if((get_ticks() - ticks) > 10) {
            ticks = get_ticks();
            debug_num(PID, DECIMAL);
        }

        KeyEvent event = kb_pollevent();

        if(event.type == KB_PRESS && event.keycode == KEY_ESCAPE) {
            proc_exit();
        }

    }
}