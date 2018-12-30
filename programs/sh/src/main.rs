#![no_std]
#![no_main]
#![feature(asm, start)]

#[macro_use]
extern crate rost_std;

use rost_std::keyboard::*;
use rost_std::vga::*;

#[no_mangle]
#[start]
pub extern "C" fn _start() {

    let kb = Keyboard::get();
    let mut vga = VGA::try_get().unwrap();

    loop {

        if let Some(event) = kb.poll_event() {
            if event.get_kind() == EventKind::Press {
                vga.write_char(10, 10, event.get_ascii(KeyCase::Lower), ColorCode::new(Color::Black, Color::Blue));
                vga.show();
            }
        }
    }

    
}


