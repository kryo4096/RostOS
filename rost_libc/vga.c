#include "std.h"
#include "syscall.h"
#include "vga.h"

#include "std.h"
#include "syscall.h"
#include "port_io.h"

const uint64_t VGA_WIDTH = 80;
const uint64_t VGA_HEIGHT = 25;

static volatile uint16_t* VGA_ADDRESS;
static uint16_t VGA_BUFFER[2000]; 

void vga_init() {
    VGA_ADDRESS = (volatile uint16_t*) map_vga();
    vga_clear();
    vga_show();
}

void vga_drop() {
    vga_clear();
    vga_show();
    unmap_vga();
}

void write_char(uint64_t x, uint64_t y, char ascii_byte, uint8_t color_code) {
    if(x >= VGA_WIDTH || y >= VGA_HEIGHT) {
        return;
    }

    VGA_BUFFER[VGA_WIDTH * y + x] = (color_code << 8) | ascii_byte & 0x00ff;
}


uint16_t read_char(uint64_t x, uint64_t y) {
    if(x >= VGA_WIDTH || y >= VGA_HEIGHT) {
        return 0;
    }

    return VGA_BUFFER[VGA_WIDTH * y + x];
}


uint8_t create_color_code(Color background, Color foreground) {
    return (background << 4) | (foreground & 0x0f);
}

void vga_clear() {   
    for(volatile size_t offset = 0; offset < VGA_HEIGHT * VGA_WIDTH; offset++) {
        VGA_BUFFER[offset] = 0;
    }
}

void vga_show() {
    for(size_t offset = 0; offset < VGA_HEIGHT * VGA_WIDTH; offset++) {
        *(VGA_ADDRESS + offset) = VGA_BUFFER[offset];
    }
}

void set_cursor(int x, int y)
{
	uint16_t pos = y * VGA_WIDTH + x;
 
	outb(0x3D4, 0x0F);
	outb(0x3D5, (uint8_t) (pos & 0xFF));
	outb(0x3D4, 0x0E);
	outb(0x3D5, (uint8_t) ((pos >> 8) & 0xFF));
}