#![no_std]
#![no_main]

#![feature(const_fn)]

extern crate bootloader_precompiled;
extern crate spin;

#[macro_use]
mod vga;
mod panic;

pub use panic::panic;

use vga::*;


#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World!");

    loop {}

}

