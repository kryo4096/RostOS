#include "terminal.h"
#include "vga.h"
#include "keyboard.h"

static int cursor = 0;
static uint8_t color_code = C_BLACK << 4 | C_WHITE;

void _update() {
    vga_show();
    vga_setcursor(cursor, 24, color_code);
}


void _putchar(char c) {
    if(c == '\n' || cursor == 80) {
            vt_newln();
        } else {
            vga_wrchar(cursor, 24, c, color_code);
            cursor++;
    }
}

void _delchar() {
    cursor--;  
    vga_wrchar(cursor, 24, 0, color_code);
}

void _print(char *string) {
    for (int i = 0; string[i]; i++) {
        _putchar(string[i]);
    }
}

void _newln() {
    
    for (int y = 0; y < 24; y++) {
        for (int x = 0; x < 80; x++) {
            vga_wrchar(x, y, vga_rdchar(x, y+1), color_code);
        }
    }
    
    for (int x = 0; x < 80; x++) {
        vga_wrchar(x, 24, 0, color_code);
    }

    cursor = 0;
}

void vt_init() {
    vga_init();
    vga_fill(color_code);
    _update();
}

void vt_putchar(char c) {
    _putchar(c);
    _update();
}

void vt_delchar() {
    _delchar();
    _update();
}

void vt_newln() {
    _newln();
    _update();
}

void vt_print(char *string) {
    _print(string);
    _update();
}

void vt_println(char *string) {
    _print(string);
    _newln();
    _update();
}

void vt_clear() {
    vga_fill(color_code);
    cursor = 0;
    _update();
}

void vt_set_color(uint8_t _color_code) {
    color_code = _color_code;
}

char* vt_readln(char* prompt) {
    static char line[81];

    for(int i = 0; i < 81; i++) {
        line[i] = 0;
    }

    static KeyCase key_case = KB_CASE_LOWER;

    uint8_t c;

    vt_print(prompt);

    int line_start = cursor;
    int line_index = 0;
    
    do {
        KeyEvent evt = kb_pollevent();
        c = kb_getchar(evt, key_case);

        if(evt.keycode == KEY_LEFT_SHIFT) {
            if(evt.type == KB_PRESS){
                key_case = KB_CASE_UPPER;
            } else {
                key_case = KB_CASE_LOWER;
            }
            continue;
        }

        if(c == '\b' && cursor > line_start) {
           vt_delchar();
           line_index--;
           line[line_index] = 0;
           continue;
        }


        if(c == 0 || c == '\n' || c == '\b') {
            continue;
        }

        line[line_index] = c;
        line_index++;

        vt_putchar(c);
        
    } while(c != '\n' && cursor < 80);

    line[cursor] = 0;

    if(line_index == 0) {
        return 0;
    }

    return (char*) (&line);
}

