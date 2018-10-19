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
mod consts;
mod syscall;

pub use panic::panic;

use x86_64::structures::paging::*;
use x86_64::{VirtAddr, PhysAddr};

pub const TEST_CODE_ADDR: *mut [u8; 4096] = (0) as *mut _;

global_asm!(include_str!("routines.S"));

extern "C" {
    fn int_80();
    fn _call(addr: u64);
    fn _switch(stack_addr: u64, lower_addr: u64);
}

pub fn io_wait() {
    let mut port = x86_64::instructions::port::Port::new(0x80);
    unsafe {
        port.write(0u8);
    }
}

#[no_mangle]
pub extern "C" fn kstart() -> ! {
    let mut p4 = unsafe { &mut *(consts::P4_TABLE_ADDR as *mut PageTable) };
    let mut rec = RecursivePageTable::new(p4).unwrap();

    kmain();

}

#[no_mangle]
pub extern "C" fn kmain() -> ! {

    let info = ::boot_info::get_info();

    boot_info::print_map();
    let mut frame_allocator;

    unsafe {
        frame_allocator = memory::init();
        interrupt::init();
    };

    let mut p4 = unsafe { &mut *(consts::P4_TABLE_ADDR as *mut PageTable) };

    let mut rec = RecursivePageTable::new(p4).unwrap();

    
    memory::debug_page_table();

    rec.map_to(
        Page::containing_address(VirtAddr::from_ptr(TEST_CODE_ADDR)),
        frame_allocator.alloc().unwrap(),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        &mut frame_allocator,
    )
    .unwrap()
    .flush();


    let program = include_bytes!("../program"); 

    unsafe {
        for i in 0..program.len() {
            (*TEST_CODE_ADDR)[i] = program[i];
        }
        _call(TEST_CODE_ADDR as u64);
    }

    loop{}

}
