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

pub use panic::panic;

use x86_64::structures::paging::*;
use x86_64::{VirtAddr, PhysAddr};

pub const TEST_CODE_ADDR: *mut [u8; 4096] = (100 * consts::PAGE_SIZE) as *mut _;

global_asm!(include_str!("routines.S"));

extern "C" {
    fn int_80();
    fn _call(addr: u64);
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

static HELLO : usize = 6;

#[no_mangle]
pub extern "C" fn kmain() -> !{

    let info = ::boot_info::get_info();

    let [rsp,rip] : [u64;2];

    unsafe {
        asm!("mov $0, rsp":"=r"(rsp):::"intel");
        asm!("lea $0, [rip+0]":"=r"(rip):::"intel");
    }

    println!("rsp=0x{:x}, rip=0x{:x}", rsp, rip);

    let mut frame_allocator;

    unsafe {
        frame_allocator = memory::init();
        interrupt::init();
    };

    let mut p4 = unsafe { &mut *(consts::P4_TABLE_ADDR as *mut PageTable) };

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

    let program = include_bytes!("program"); 

    unsafe {
        for i in 0..program.len() {
            (*TEST_CODE_ADDR)[i] = program[i];
        }
        _call(TEST_CODE_ADDR as u64);
    }

    loop{}

}
