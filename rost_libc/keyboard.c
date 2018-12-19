#include "keyboard.h"

const char BACKSPACE = '\b';
const char ESCAPE = '\e';

const char KEYMAP_LOWER[] = {
    0,
    '\e',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    '0',
    '\'',
    '^',
    '\b',
    '\t',
    'q',
    'w',
    'e',
    'r',
    't',
    'z',
    'u',
    'i',
    'o',
    'p',
    '[',
    ']',
    '\n',
    0,
    'a',
    's',
    'd',
    'f',
    'g',
    'h',
    'j',
    'k',
    'l',
    0,
    '{',
    0,
    0,
    '$',
    'y',
    'x',
    'c',
    'v',
    'b',
    'n',
    'm',
    ',',
    '.',
    '-',
    0,
    0,
    0,
    ' ',
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};

const char KEYMAP_UPPER[] = {
    0,
    0,
    '+',
    '"',
    '*',
    0,
    '%',
    '&',
    '/',
    '(',
    ')',
    '=',
    '?',
    '`',
    0,
    '\t',
    'Q',
    'W',
    'E',
    'R',
    'T',
    'Z',
    'U',
    'I',
    'O',
    'P',
    0,
    '!',
    '\n',
    0,
    'A',
    'S',
    'D',
    'F',
    'G',
    'H',
    'J',
    'K',
    'L',
    0,
    '{',
    0,
    0,
    '}',
    'Y',
    'X',
    'C',
    'V',
    'B',
    'N',
    'M',
    ';',
    ':',
    '_',
    0,
    0,
    0,
    ' ',
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};

char kb_getchar(KeyEvent event, KeyCase key_case) {
    if(event.type != KB_PRESS) {
        return 0;
    }

    switch(key_case) {
    case KB_CASE_LOWER: return KEYMAP_LOWER[event.keycode];
    case KB_CASE_UPPER: return KEYMAP_UPPER[event.keycode];
    }

}

void kb_wait_any() {
    while(kb_pollevent().type != KB_PRESS);
}

KeyEvent kb_pollevent() {
    KeyEvent key_event;

    uint8_t scancode = get_scancode();
    if(scancode == 0) {
        key_event.type = KB_NONE;
        key_event.keycode = 0; 
    } else if(scancode > 0x80) {
        key_event.type = KB_RELEASE;
        key_event.keycode = scancode - 0x80;
    } else {
        key_event.type = KB_PRESS;
        key_event.keycode = scancode;
    }

    return key_event;
}

