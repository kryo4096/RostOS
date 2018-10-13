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
mod memory;
mod panic;
mod time;

pub use panic::panic;

use bootloader_precompiled::bootinfo::BootInfo;
use vga::*;

use x86_64::structures::paging::*;
use x86_64::VirtAddr;

pub const TEST_CODE_ADDR: *mut [u8; 4096] = (100 * memory::PAGE_SIZE) as *mut _;

global_asm!(include_str!("routines.S"));

extern "C" {
    fn int_80();
    fn test_code();
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
    let mut frame_allocator;

    unsafe {
        frame_allocator = memory::init();
        interrupt::init();
    };

    let mut p4 = unsafe { &mut *(0xf_fff_fff_fff_fff_000 as *mut PageTable) };
    let mut rec = RecursivePageTable::new(p4).unwrap();

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let frame = frame_allocator.alloc().expect("no more frames!");

    rec.map_to(
        Page::containing_address(VirtAddr::from_ptr(TEST_CODE_ADDR)),
        frame,
        flags,
        &mut frame_allocator,
    )
    .unwrap()
    .flush();

    unsafe {
        *TEST_CODE_ADDR = [0;4096];
        (*TEST_CODE_ADDR)[0] = 0xCA;
        test_code();
    }

    println!("woah");

    loop {}
}
