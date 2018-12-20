// programmed by Nils Leuzinger

#include "std.h"
#include "process.h"
#include "terminal.h"
#include "keyboard.h"
#include "vga.h"

bool is_within_bounds(int x, int y, int MAX_X, int MAX_Y) {
	return x >= 0
		&& y >= 0
		&& x < MAX_X
		&& y < MAX_Y;
}

int count_live_neighbours_around_cell(int32_t *arr, int x, int y, int width, int height) {
	int neighbours = 0;
	for (int i = x - 1; i <= x + 1; i++) {
		for (int j = y - 1; j <= y + 1; j++) {
			if (i == x && j == y) {
				continue;
			}
			if (is_within_bounds(i, j, width, height)
					&& arr[i * height + j] == 1) {
				neighbours++;
			}
		}
	}
	return neighbours;
}

void _start() {
    // Initialize a new virtual terminal. There is no stdin / stdout yet.
    vga_init();

    const int32_t BUFFERS = 2;
    const int64_t width = VGA_WIDTH;
    const int64_t height = VGA_HEIGHT;

    int32_t current_buffer = 0; // use a highly advanced double buffering strategy
    int32_t habitats[BUFFERS][width][height];
    for (int64_t h = 0; h < BUFFERS; h++) {
	    for (int64_t i = 0; i < width; i++) {
		    for (int64_t j = 0; j < height; j++) {
			    habitats[h][i][j] = 0;
			    habitats[h][i][j] = 0;
		    }
	    }
    }

    habitats[current_buffer][40][40] = 1;
    habitats[current_buffer][40][42] = 1;
    habitats[current_buffer][39][42] = 1;
    habitats[current_buffer][42][41] = 1;
    habitats[current_buffer][43][42] = 1;
    habitats[current_buffer][44][42] = 1;
    habitats[current_buffer][45][42] = 1;

    /*const uint8_t testcolor = vga_color_code(C_BLACK, C_WHITE);
    vga_clear();
    for (int64_t i = 0; i < width; i++) {
	    for (int64_t j = 0; j < height; j++) {
		    if (habitats[current_buffer][i][j] != 0) {
			    vga_wrchar(i, j, 254, testcolor);
		    }
	    }
    }
    vga_show();
    */
    uint64_t time = get_ticks();
    uint64_t new_time = time;

    while(1) {
	    KeyEvent event = kb_pollevent();

	    if (event.keycode == KEY_ESCAPE) {
		    break;
	    }

	    new_time = get_ticks();
	    if (new_time - time > 10) {
		    // do gametick
		    time = new_time;

		    int32_t previous_buffer = current_buffer;
		    current_buffer++;
		    current_buffer %= BUFFERS;

		    for (int64_t i = 0; i < width; i++) {
			    for (int64_t j = 0; j < height; j++) {
				    // calculate what happens to a cell in its habitat
				    uint8_t neighbours = count_live_neighbours_around_cell(habitats[previous_buffer], i, j, width, height);
				    if (habitats[previous_buffer][i][j] == 0) {
					    if (neighbours == 3) {
						    habitats[current_buffer][i][j] = 1;
					    } else {
						    habitats[current_buffer][i][j] = 0;
					    }
				    } else {
					    if (neighbours < 2 || 3 < neighbours) {
						    habitats[current_buffer][i][j] = 0;
					    } else {
						    habitats[current_buffer][i][j] = 1;
					    }
				    }
			    }
		    }

		    const uint8_t COLOR = vga_color_code(C_WHITE, C_WHITE);
		    vga_clear();
		    for (int64_t i = 0; i < width; i++) {
			    for (int64_t j = 0; j < height; j++) {
				    if (habitats[current_buffer][i][j] != 0) {
					    vga_wrchar(i, j, 254, COLOR);
				    }
			    }
		    }
		    vga_show();
	    }
    }

    // Exit the process. Never forget this.
    proc_exit();
}
