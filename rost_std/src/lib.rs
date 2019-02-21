#![macro_export]
#![feature(asm, global_asm, naked_functions)]
#![no_std]

use core::panic::PanicInfo;

pub mod keyboard;
pub mod vga;
#[macro_use]
mod syscall;
pub mod ascii;
#[macro_export]
pub mod debug;
pub mod memory;
pub mod process;
pub mod signal;
pub mod time;
pub mod misc;
pub mod port;

#[no_mangle]
#[panic_handler]
pub fn panic(panic_info: &PanicInfo) -> ! {
    kprintln!("Panic! \n {:?}", panic_info);

    process::exit();
}
