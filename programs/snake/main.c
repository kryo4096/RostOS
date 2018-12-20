#include "std.h"
#include "process.h"
#include "terminal.h"
#include "keyboard.h"
#include "vga.h"
#include "time.h"

#define MAX_SNAKE_LEN 1000
#define FRAME_DELTA 1

typedef enum dir {
    LEFT,
    UP,
    RIGHT,
    DOWN,
} dir_t;

typedef struct position {
    uint64_t x;
    uint64_t y;
} pos_t;

void _start() {
    
    vga_init();

    pos_t food;
    
    food.x = 10;
    food.y = 2;

    // snake length
    int snake_len = 1;

    pos_t snake[MAX_SNAKE_LEN] = {{VGA_WIDTH / 2, VGA_HEIGHT / 2}};
    
    dir_t direction = UP;
    
    time_t start_time = time_now();
    time_t last_frame = start_time;

    uint16_t color_code = vga_color_code(C_BLACK, C_GREEN);

    while(1) {
        time_t current_time = time_now();
        time_t delta = current_time - last_frame;

        if(delta > FRAME_DELTA) {
            for(int i = snake_len; i >= 0; i--) {
                snake[i + 1] = snake[i];
            }

            switch(direction) {
                case LEFT: 
                    snake[0].x = snake[1].x - 1;
                    break;
                case RIGHT:
                    snake[0].x = snake[1].x + 1;
                    break;
                case UP:
                    snake[0].y = snake[1].y - 1;
                    break;
                case DOWN:
                    snake[0].y = snake[1].y + 1;
                    break;
            }

            for(int i = 0; i < snake_len; i++) {
                vga_wrchar(snake[i].x, snake[i].y, 'o', color_code);
            }
        }

        
    }


    proc_exit();
}