#include "syscall.h"
#include "std.h"
#include "keyboard.h"
#include "vga.h"

static size_t cursor = 0;

void new_line() {

    for (int y = 0; y < 24; y++) {
        for (int x = 0; x < 80; x++) {
            write_char(x, y, read_char(x, y+1), create_color_code(C_BLACK, C_LIGHT_GREEN));
        }
    }
    
    for (int x = 0; x < 80; x++) {
        write_char(x, 24, 0, create_color_code(C_BLACK, C_LIGHT_GREEN));
    }

    cursor = 0;
    set_cursor(cursor, 24);
}


void print_line(char* str) {
    
    
    for(int i = 0; str[i]; i++) {
        write_char(cursor, 24, str[i], create_color_code(C_BLACK, C_LIGHT_GREEN));
        cursor++;

        if(cursor == 80 || str[i] == '\n') {
            new_line();
            vga_show();
        }
    }
    set_cursor(cursor, 24);
    vga_show();
}

const char* PROMPT = "> ";

char* read_line() {
    static char line[81] = {};
    static KeyCase key_case = CASE_LOWER;


    uint8_t c;
    
    do {
        KeyEvent evt = get_key_event(get_scancode());

        c = get_char(evt, key_case);

        if(c == '\b' && cursor > 0) {
            cursor--;
            write_char(cursor, 24, 0, create_color_code(C_BLACK, C_LIGHT_GREEN));
        } else if(evt.keycode == KEY_LEFT_SHIFT) {
            if(evt.type == KEY_EV_PRESS){
                key_case = CASE_UPPER;
            } else {
                key_case = CASE_LOWER;
            }
        }
        else if(c == 0 || c == '\n' || c == '\b') {
            
        } else {
            line[cursor] = c;
            write_char(cursor, 24, c, create_color_code(C_BLACK, C_LIGHT_GREEN));
            cursor++;
        }
        set_cursor(cursor, 24);
        vga_show();

    } while(c != '\n' && cursor < 80);

    line[cursor] = 0;

    return (char*) (&line);
}

void _start() {
    vga_init();

    while(1) {
        char* line = read_line();
        new_line();
        vga_show();
        uint64_t pid = execute(line);
        if (pid) {
            wait_pid(pid);

        } else {
            print_line("Command not found: ");
            print_line(line);
            new_line();
        }
    }
}
