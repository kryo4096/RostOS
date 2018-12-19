#include "std.h"

void pio_outb(uint16_t port, uint8_t val);

void pio_outw(uint16_t port, uint16_t val);

void pio_outl(uint16_t port, uint32_t val);

uint8_t pio_inb(uint16_t port);

uint16_t pio_inw(uint16_t port);

uint32_t pio_inl(uint16_t port);