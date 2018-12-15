#include "std.h"
#include "syscall.h"
#include "vga.h"
#include "keyboard.h"


void _start() {

    vga_init();

    // ball position
    int64_t x = VGA_WIDTH / 2, y = VGA_HEIGHT / 2;

    // ball velocity
    int64_t u = 1, v = 1;
    
    // paddle positions
    int64_t paddle_left = VGA_HEIGHT / 2, paddle_right = VGA_HEIGHT / 2;

    // paddle velocities
    int64_t paddle_left_v = 0, paddle_right_v = 0;

    // current time (for game loop)
    int64_t time = get_ticks();

    int64_t i = 0;
    
    // current scores (a bit hacky)
    char score_left = '0', score_right = '0';

    while(1) {

        // get a key event
        KeyEvent event = get_key_event(get_scancode());

        if(event.keycode == KEY_ESCAPE) {
            break;
        }

        // W/S control left paddle, UP/DOWN control right_paddle
        if(event.type == KEY_EV_PRESS) {
            if(event.keycode == KEY_W) {
                paddle_left_v = -1;
            }
            if(event.keycode == KEY_S) {
                paddle_left_v = 1;
            }
            if(event.keycode == KEY_UP) {
                paddle_right_v = -1;
            }
            if(event.keycode == KEY_DOWN) {
                paddle_right_v = 1;
            }
        }

        // handle key releases
        if(event.type == KEY_EV_RELEASE) {
            if(event.keycode == KEY_W) {
                paddle_left_v = 0;
            }
            if(event.keycode == KEY_S) {
                paddle_left_v = 0;
            }
            if(event.keycode == KEY_UP) {
                paddle_right_v = 0;
            }
            if(event.keycode == KEY_DOWN) {
                paddle_right_v = 0;
            }
        }

        // update "physics" if at least one tick has passed since last iteration
        uint64_t new_ticks = get_ticks();
        if (new_ticks - time > 10) {
            i++;
            // reset physics timer
            time = new_ticks;

            if(i%3==0) {

                // check for vertical reflection
                if(y == VGA_HEIGHT - 1 || y == 0)  {
                    v *= -1;
                }

                if(x == 0) {
                    if(y - paddle_left <= 1 && y - paddle_left >= -1) {
                        u *= -1;
                    } else {
                        x = VGA_WIDTH / 2;
                        y = VGA_HEIGHT / 4 + get_ticks() % (VGA_HEIGHT / 2);
                         if(score_right == '9') {
                                                    break;
                                                }
                        score_right++;
                        u *=-1 ;
                    }
                }

                if(x == VGA_WIDTH - 1) {
                    if(y - paddle_right <= 1 && y - paddle_right >= -1) {
                        u *= -1;
                    } else {
                        x = VGA_WIDTH / 2;
                        y = VGA_HEIGHT / 4 + get_ticks() % (VGA_HEIGHT / 2);
                        if(score_left == '9') {
                            break;
                        }
                        score_left++;



                        u *= -1;
                    }
                }

                x += u;
                y += v;
            }

            paddle_left = (paddle_left + paddle_left_v);
            paddle_right = (paddle_right + paddle_right_v);
        }


        const uint8_t BALL_COLOR = create_color_code(C_BLACK, C_WHITE);
        const uint8_t PADDLE_COLOR = create_color_code(C_BLACK, C_WHITE);
        const uint8_t TEXT_COLOR = create_color_code(C_BLACK, C_WHITE);
        
        // Clear Buffer
        vga_clear();

        // Draw Ball 
        write_char((uint64_t)x, (uint64_t)y, 254, BALL_COLOR);
        
        // Draw Left Paddle
        write_char((uint64_t)0, (uint64_t)paddle_left, 221, PADDLE_COLOR);
        write_char((uint64_t)0, (uint64_t)paddle_left+1, 221, PADDLE_COLOR);
        write_char((uint64_t)0, (uint64_t)paddle_left-1, 221, PADDLE_COLOR);

        // Draw Right Paddle
        write_char((uint64_t)VGA_WIDTH-1, (uint64_t)paddle_right, 222, PADDLE_COLOR);
        write_char((uint64_t)VGA_WIDTH-1, (uint64_t)paddle_right+1, 222, PADDLE_COLOR);
        write_char((uint64_t)VGA_WIDTH-1, (uint64_t)paddle_right-1, 222, PADDLE_COLOR);

        // Draw Scores
        write_char((uint64_t)VGA_WIDTH/2 -5, 2, score_left, TEXT_COLOR);
        write_char((uint64_t)VGA_WIDTH/2 +5, 2, score_right, TEXT_COLOR);

        // Flip Buffers
        vga_show();


    }

    vga_drop();
    proc_exit();
}