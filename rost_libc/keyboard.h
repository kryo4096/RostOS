#ifndef KEYBOARD_H 
#define KEYBOARD
#include "std.h"
#include "syscall.h"

#define KEY_A 0x1E
#define KEY_B 0x30
#define KEY_C 0x2E
#define KEY_D 0x20
#define KEY_E 0x12
#define KEY_F 0x21
#define KEY_G 0x22
#define KEY_H 0x23
#define KEY_I 0x17
#define KEY_J 0x24
#define KEY_K 0x25
#define KEY_L 0x26
#define KEY_M 0x32
#define KEY_N 0x31
#define KEY_O 0x18
#define KEY_P 0x19
#define KEY_Q 0x10
#define KEY_R 0x13
#define KEY_S 0x1F
#define KEY_T 0x14
#define KEY_U 0x16
#define KEY_V 0x2F
#define KEY_W 0x11
#define KEY_X 0x2D
#define KEY_Y 0x15
#define KEY_Z 0x2C
#define KEY_1 0x02
#define KEY_2 0x03
#define KEY_3 0x04
#define KEY_4 0x05
#define KEY_5 0x06
#define KEY_6 0x07
#define KEY_7 0x08
#define KEY_8 0x09
#define KEY_9 0x0A
#define KEY_0 0x0B
#define KEY_F1 0x3B
#define KEY_F2 0x3C
#define KEY_F3 0x3D
#define KEY_F4 0x3E
#define KEY_F5 0x3F
#define KEY_F6 0x40
#define KEY_F7 0x41
#define KEY_F8 0x42
#define KEY_F9 0x43
#define KEY_F10 0x44
#define KEY_F11 0x57
#define KEY_F12 0x58
#define KEY_ESC 0x01
#define KEY_ENTER 0x1C
#define KEY_SPACE 0x39
#define KEY_TAB 0x0F
#define KEY_BACKSPACE 0x0E
#define KEY_CAPSLOCK 0x3A
#define KEY_ESCAPE 0x01
#define KEY_UP 0x48 
#define KEY_DOWN 0x50
#define KEY_LEFT 0x4B
#define KEY_RIGHT 0x4D

#define KEY_LEFT_SHIFT 0x2A
#define KEY_LEFT_CTRL 0x1D
#define KEY_RIGHT_SHIFT 0x36
#define KEY_NUMBER_LOCK 0x45
#define KEY_SCROLL_LOCK 0x46

#define KEY_KEYPAD_PLUS 0x4E
#define KEY_KEYPAD_MINUS 0x4A
#define KEY_KEYPAD_1 0x4F
#define KEY_KEYPAD_2 0x50
#define KEY_KEYPAD_3 0x51
#define KEY_KEYPAD_4 0x4B
#define KEY_KEYPAD_5 0x4C
#define KEY_KEYPAD_6 0x4D
#define KEY_KEYPAD_7 0x47
#define KEY_KEYPAD_8 0x48
#define KEY_KEYPAD_9 0x49
#define KEY_KEYPAD_0 0x52


typedef enum KeyEventType {
    KB_PRESS,
    KB_RELEASE,
    KB_NONE,
} KeyEventType;

typedef struct KeyEvent {
    uint8_t keycode;
    KeyEventType type;
} KeyEvent;

KeyEvent kb_pollevent();

typedef enum KeyCase {
    KB_CASE_LOWER,
    KB_CASE_UPPER,
} KeyCase;

void kb_wait_any();

char kb_getchar(KeyEvent event, KeyCase key_case);



#endif