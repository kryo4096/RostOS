#![no_std]
#![no_main]
#![feature(start)]


#[macro_use]
extern crate rost_std;

use rost_std::process;
use rost_std::signal;
use rost_std::vga;
use rost_std::vga::{ColorCode, Color, VGA_WIDTH, VGA_HEIGHT};
use rost_std::keyboard;
use rost_std::keyboard::{KeyEvent, EventKind, KEY_DOWN, KEY_UP, KEY_W, KEY_S, KEY_ESCAPE};
use core::sync::atomic::*;

static P1_AXIS : AtomicIsize = AtomicIsize::new(0);
static P2_AXIS : AtomicIsize = AtomicIsize::new(0);
static RUNNING : AtomicBool = AtomicBool::new(true);

extern "C" fn keyboard_handler(scancode: u64, _:u64, _:u64,_:u64) {
    if let Some(event) = KeyEvent::from_scancode(scancode as u8) {
        match event.kind() {
            EventKind::Press => {
                match event.keycode() {
                    KEY_S => P1_AXIS.store(1, Ordering::SeqCst),
                    KEY_W => P1_AXIS.store(-1, Ordering::SeqCst),
                    KEY_DOWN => P2_AXIS.store(1, Ordering::SeqCst),
                    KEY_UP => P2_AXIS.store(-1, Ordering::SeqCst),
                    KEY_ESCAPE => RUNNING.store(false, Ordering::SeqCst),
                    _ => (),
                };
            },
            EventKind::Release => {
                match event.keycode() {
                    KEY_S => P1_AXIS.store(0, Ordering::SeqCst),
                    KEY_W => P1_AXIS.store(0, Ordering::SeqCst),
                    KEY_DOWN => P2_AXIS.store(0, Ordering::SeqCst),
                    KEY_UP => P2_AXIS.store(0, Ordering::SeqCst),
                    _ => (),
                };
            }
        }
    }


}

struct Vec2 {
    x: isize,
    y: isize,
}

trait GameObject {
    fn render(&self);
    fn update(&mut self);
}

struct Paddle {
    pos: Vec2,
    color: ColorCode,
}

impl GameObject for Paddle {
    fn render(&self) {
        for i in -1isize..1 {
            vga::write_char(self.pos.x as _, (self.pos.y + i) as _, b' ', self.color);
        }
    }

    fn update(&mut self) {}
}

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    color: ColorCode,
}

impl GameObject for Ball {
    fn render(&self) {
        vga::write_char(self.pos.x as _, self.pos.y as _, 254, self.color);
    }

    fn update(&mut self) {
        if !(self.pos.y < VGA_HEIGHT as _ && self.pos.y > 0) {
            self.vel.y *= -1;
        }

        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }
}


#[start]
#[no_mangle]
fn _start() {
    vga::map();
    vga::clear();
    

    signal::subscribe(signal::SIGNAL_KEYBOARD, keyboard_handler);

    let mut ball = Ball {
        pos: Vec2 {x: (VGA_WIDTH / 2) as _, y: (VGA_HEIGHT / 2) as _},
        vel: Vec2 {x: 1, y: 1},
        color: ColorCode::new(Color::Black, Color::White)
    };

    let mut paddle_1 = Paddle {
        pos: Vec2 {x: 0, y: (VGA_HEIGHT / 2) as _},
        color: ColorCode::new(Color::Blue, Color::Blue)
    };

    let mut paddle_2 = Paddle {
        pos: Vec2 {x: (VGA_WIDTH - 1) as _, y: (VGA_HEIGHT / 2) as _},
        color: ColorCode::new(Color::Red, Color::Red)
    };

    while RUNNING.load(Ordering::SeqCst) {
        
        ball.update();
        paddle_1.update();
        paddle_2.update();

        if ball.pos.x <= 0 {
            if (ball.pos.y - paddle_1.pos.y).abs() <= 1 {
                ball.vel.x *= -1;
            } else {
                ball.pos.x = (VGA_WIDTH / 2) as _;
            }
        }
        
        if ball.pos.x >= (VGA_WIDTH - 1) as _ {
            if (ball.pos.y - paddle_2.pos.y).abs() <= 1 {
                ball.vel.x *= -1;
            } else {
                ball.pos.x = (VGA_WIDTH / 2) as _;
            }
        }

        paddle_1.pos.y += P1_AXIS.load(Ordering::SeqCst);
        paddle_2.pos.y += P2_AXIS.load(Ordering::SeqCst);

        vga::clear();
        ball.render();
        paddle_1.render();
        paddle_2.render();

        vga::show();
        

        process::sleep(10);
    }
    process::exit();
}
