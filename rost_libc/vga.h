#include "std.h"
#include "syscall.h"


const uint64_t VGA_WIDTH;
const uint64_t VGA_HEIGHT;

typedef enum Color {
    C_BLACK = 0x0,
    C_BLUE = 0x1,
    C_GREEN = 0x2,
    C_CYAN = 0x3,
    C_RED = 0x4,
    C_MAGENTA = 0x5,
    C_BROWN = 0x6,
    C_LIGHT_GRAY = 0x7,
    C_DARK_GRAY = 0x8,
    C_LIGHT_BLUE = 0x9,
    C_LIGHT_GREEN = 0xa,
    C_LIGHT_CYAN = 0xb,
    C_LIGHT_RED = 0xc,
    C_LIGHT_MAGENTA = 0xd,
    C_YELLOW = 0xe,
    C_WHITE = 0xf
} Color;

void vga_clear();
void write_char(uint64_t x, uint64_t y, char ascii_byte, uint8_t color_code);
uint16_t read_char(uint64_t x, uint64_t y);
uint8_t create_color_code(Color background, Color foreground);

void set_cursor(int x, int y);

void vga_init();
void vga_drop();
void vga_show();

