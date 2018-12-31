#![no_std]
#![no_main]
#![feature(start)]


#[macro_use]
extern crate rost_std;

use rost_std::process;
use rost_std::signal;

extern "C" fn handle_kb(scancode:u64,_:u64,_:u64,_:u64) {
    kprint!("scancode: 0x{:x}", scancode);
}



#[start]
#[no_mangle]
fn _start() {
    signal::subscribe(0, handle_kb);
    loop {}
}
