#![feature(asm, naked_functions)]
#![no_std]

use core::panic::PanicInfo;

pub mod vga;

#[macro_use]
mod syscall;

#[no_mangle]
#[panic_handler]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {
    loop {}
}


