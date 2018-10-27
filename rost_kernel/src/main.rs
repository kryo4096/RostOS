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
    naked_functions
)]
#![allow(unused)]

extern crate alloc;
extern crate bootloader;
extern crate linked_list_allocator;
extern crate spin;
extern crate x86_64;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga;
mod boot_info;
mod consts;
mod interrupt;
mod memory;
mod panic;
mod process;
mod syscall;
mod time;

pub use panic::panic;

use x86_64::structures::paging::*;
use x86_64::ux::u9;
use x86_64::{PhysAddr, VirtAddr};

use memory::frame_allocator::FrameStackAllocator;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use consts::*;

pub const TEST_CODE_ADDR: *mut [u8; 4096] = USER_START as _;

global_asm!(include_str!("routines.S"));

extern "C" {
    fn int_80();
    fn proc_start(entry_point: u64, stack_ptr: u64);
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

    unsafe {
        interrupt::init();
    }

    let mut frame_allocator = unsafe { memory::init() };
    let mut p4 = memory::get_p4();

    let heap_start = Page::<Size4KiB>::containing_address(VirtAddr::new(KERNEL_HEAP_START));
    let heap_end = Page::<Size4KiB>::containing_address(VirtAddr::new(
        KERNEL_HEAP_START + KERNEL_HEAP_SIZE - 1,
    ));

    // map kernel heap
    for page in Page::range(heap_start, heap_end) {
        p4.map_to(
            page,
            frame_allocator.alloc().unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            &mut frame_allocator,
        )
        .expect("Unable to map kernel heap")
        .flush();
    }

    // map vga buffer to higher memory
    p4.map_to(
        Page::<Size4KiB>::containing_address(VirtAddr::new(VGA_BUFFER_VADDR)),
        PhysFrame::containing_address(PhysAddr::new(VGA_BUFFER_PADDR)),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        &mut frame_allocator,
    )
    .expect("Unable to map VGA buffer");

    unsafe {
        ALLOCATOR
            .lock()
            .init(KERNEL_HEAP_START as usize, KERNEL_HEAP_SIZE as usize);
    }

    unsafe {
        syscall::init();
    }
    kmain(frame_allocator);
}

#[no_mangle]
pub extern "C" fn kmain(mut frame_allocator: FrameStackAllocator) -> ! {
    run_program(&mut frame_allocator, include_bytes!("../program"))
}



pub fn run_program(frame_allocator: &mut FrameStackAllocator, program: &'static [u8]) -> ! {
    let mut p4 = memory::get_p4();

    // map program code
    p4.map_to(
        Page::containing_address(VirtAddr::from_ptr(TEST_CODE_ADDR)),
        frame_allocator.alloc().unwrap(),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        frame_allocator,
    )
    .unwrap()
    .flush();

    // map program stack
    for page in Page::range(
        Page::containing_address(VirtAddr::new(USER_STACK_TOP - USER_STACK_SIZE)),
        Page::<Size4KiB>::containing_address(VirtAddr::new(USER_STACK_TOP)),
    ) {
        p4.map_to(
            page,
            frame_allocator.alloc().unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            frame_allocator,
        )
        .unwrap()
        .flush();
    }

    unsafe {
        for (i, byte) in program.iter().enumerate() {
            (*TEST_CODE_ADDR)[i] = *byte;
        }
        proc_start(TEST_CODE_ADDR as u64, USER_STACK_TOP);
    }

    loop {}
}
