#![feature(global_asm, naked_functions)]
#![no_std]



use core::panic::PanicInfo;


pub mod vga;
pub mod keyboard;
#[macro_use]
mod syscall;
pub mod ascii;
pub mod debug;
pub mod process;

#[no_mangle]
#[panic_handler]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {

    kprintln!("Panic! \n {:?}", panic_info);
    
    loop {}
}

