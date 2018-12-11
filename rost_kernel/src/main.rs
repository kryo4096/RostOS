#![no_std]
#![no_main]
#![feature(
    const_fn,
    global_asm,
    abi_x86_interrupt,
    integer_atomics,
    asm,
    alloc,
    alloc_error_handler,
    lang_items,
    naked_functions,
    const_vec_new,
    panic_info_message,
    const_slice_len,
    never_type
)]
#![allow(unused)]

#[macro_use]
extern crate alloc;
extern crate bootloader;
extern crate linked_list_allocator;
extern crate spin;
extern crate x86_64;
extern crate rust_fs;
extern crate xmas_elf;
extern crate volatile;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;
mod boot_info;
mod consts;
mod interrupt;
mod memory;
mod panic;
mod process;
mod syscall;
mod time;
mod random;
mod elf;
mod fs;
mod gdt;
mod keyboard;

use core::ptr;

use x86_64::structures::paging::*;
use x86_64::ux::u9;
use x86_64::{PhysAddr, VirtAddr};
use fs::*;
use alloc::vec::Vec;
use alloc::string::String;
use memory::frame_allocator::FrameStackAllocator;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

static DISK_IMAGE: &'static [u8] = include_bytes!("../disk.img");

use consts::*;

global_asm!(include_str!("routines.S"));

pub fn io_wait() {
    let mut port = x86_64::instructions::port::Port::new(0x80);
    unsafe {
        port.write(0u8);
    }
}

#[no_mangle]
pub extern "C" fn kernel_init() {
    unsafe {
        // initialize memory management
        memory::init();

        // map vga buffer to high memory
        memory::map_to_address(
            VGA_BUFFER_VADDR,
            VGA_BUFFER_PADDR,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );

        memory::unmap(VGA_BUFFER_PADDR);

        // map arbitrary frames to heap pages in high memory
        memory::map_range(
            KERNEL_HEAP_START,
            KERNEL_HEAP_START + KERNEL_HEAP_SIZE - 1,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );

        // initalize heap allocator
        ALLOCATOR
            .lock()
            .init(KERNEL_HEAP_START as usize, KERNEL_HEAP_SIZE as usize);

        gdt::init();
        // intialize interrupts (IDT, PIC)
        interrupt::init();
        fs::init();
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        //let mut buf = Vec::new();jok
        //let file = fs::open(&mut *fs::tree_mut(),"bin/init",0);
        //fs::read_file(&mut *fs::tree_mut(), file, &mut buf);

        let pid = process::create_process();

        process::load_elf_to_process(pid, b"bin/init");
        process::start_process(pid);

    }
}



