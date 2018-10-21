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
    lang_items
)]
#![allow(unused)]

extern crate bootloader;
extern crate spin;
extern crate x86_64;
extern crate alloc;
extern crate linked_list_allocator;

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
mod process;

pub use panic::panic;

use x86_64::structures::paging::*;
use x86_64::{VirtAddr, PhysAddr};
use x86_64::ux::u9;

use memory::frame_allocator::FrameStackAllocator;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

use consts::*;

pub const TEST_CODE_ADDR: *mut [u8; 4096] = USER_START as *mut _;

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

    unsafe {
        interrupt::init();
    }

    let mut frame_allocator = unsafe {memory::init()};

    let heap_start = Page::<Size4KiB>::containing_address(VirtAddr::new(KERNEL_HEAP_START));
    let heap_end = Page::<Size4KiB>::containing_address(VirtAddr::new(KERNEL_HEAP_START + KERNEL_HEAP_SIZE - 1));

    // map kernel heap
    for page in Page::range(heap_start, heap_end) { 
        rec.map_to(
            page,
            frame_allocator.alloc().unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            &mut frame_allocator,
        ).expect("Unable to map kernel heap!").flush();
    }

    // map vga buffer to higher memory
    rec.map_to(
        Page::<Size4KiB>::containing_address(VirtAddr::new(VGA_BUFFER_VADDR)),
        PhysFrame::containing_address(PhysAddr::new(0xb8000)),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        &mut frame_allocator,
    ).expect("Unable to map VGA buffer.");

    unsafe {
        ALLOCATOR.lock().init(KERNEL_HEAP_START as usize, KERNEL_HEAP_SIZE as usize);
    }

    kmain(frame_allocator);

}

#[no_mangle]
pub extern "C" fn kmain(mut frame_allocator: FrameStackAllocator) -> ! {
    let rip : u64;
    unsafe {
        asm!("lea $0, [rip+0]" : "=r"(rip) ::: "intel");
    }

    println!("0x{:x}",rip);
    
    memory::debug_page_table();
    
    let mut p4 = unsafe { &mut *(P4_TABLE_ADDR as *mut PageTable) };

    let mut rec = RecursivePageTable::new(p4).unwrap();

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

