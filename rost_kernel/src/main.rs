#![no_std]
#![no_main]
#![feature(
    const_fn,
    global_asm,
    abi_x86_interrupt,
    integer_atomics,
    asm
)]

extern crate bootloader_precompiled;
extern crate spin;
extern crate x86_64;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga;
mod boot_info;
mod interrupt;
mod panic;
mod time;
mod memory;

pub use panic::panic;

use bootloader_precompiled::bootinfo::BootInfo;
use vga::*;

use x86_64::VirtAddr;
use x86_64::structures::paging::*;

global_asm!(include_str!("routines.S"));

extern "C" {
    fn int_80();
}

pub fn io_wait() {
    let mut port = x86_64::instructions::port::Port::new(0x80);
    unsafe {
        port.write(0u8);
    }
}

pub unsafe extern "C" fn kprintln(ptr: *const u8, len: usize) {
    let slice = core::slice::from_raw_parts(ptr, len);
    let s = core::str::from_utf8_unchecked(slice);

    println!("{}", s);
}


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unsafe {
        memory::init();
        interrupt::init();
    }

    loop {}
}
