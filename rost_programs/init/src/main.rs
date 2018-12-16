#![no_main]
#![no_std]

#![feature(asm)]

extern crate rost_std;

#[no_mangle]
pub extern "C" fn _start() {
    loop {
        unsafe {
            asm!("int3");
        }
    }
}
