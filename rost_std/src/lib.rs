#![feature(asm)]
#![no_std]
// TODO
use core::panic::PanicInfo;

#[no_mangle]
#[panic_handler]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {
    loop {}
}