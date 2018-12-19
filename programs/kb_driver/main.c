#include "syscall.h"
#include "std.h"
#include "keyboard.h"

void handle_key_event(KeyEvent event) {
    
    static KeyCase current_case = CASE_LOWER;

    if(event.keycode == KEY_LEFT_SHIFT || event.keycode == KEY_RIGHT_SHIFT) {
        switch(event.type) {
            case KEY_EV_PRESS: current_case = CASE_UPPER; break;
            case KEY_EV_RELEASE: current_case = CASE_LOWER; break;
        }
        return;
    }

    if(event.keycode==KEY_ESCAPE){
        proc_exit();
    }

    char c = get_char(event, current_case);

    if(c && event.type == KEY_EV_PRESS) {
        char str[] = {c,0};
        print((char*)&str);
    }
}

void _start() {
    while(1) {
        uint8_t scancode;

        do {
            
            KeyEvent event = get_key_event(scancode);

            if(scancode) {
                handle_key_event(event);  
            } 

            scancode = get_scancode();

        }  while (scancode);

    }
}

