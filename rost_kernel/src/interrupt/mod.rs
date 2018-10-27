mod pic;
mod handler;

use x86_64::instructions::interrupts;

pub use self::pic::send_eoi;

use core::mem;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn init() {
    IDT.double_fault.set_handler_fn(handler::double_fault);
    IDT.breakpoint.set_handler_fn(handler::breakpoint);
    IDT.page_fault.set_handler_fn(handler::page_fault);
    IDT.general_protection_fault.set_handler_fn(handler::gpf);
    IDT[0x80].set_handler_fn(handler::syscall);
    IDT[0x20].set_handler_fn(handler::clock);
    IDT[0x21].set_handler_fn(handler::keyboard);

    IDT.load();
    
    pic::unmask(1); // keyboard

    pic::init();

    if !interrupts::are_enabled() {
        interrupts::enable();
    }
}
