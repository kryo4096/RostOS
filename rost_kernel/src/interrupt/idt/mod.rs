mod handler;

use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn init() {
    IDT.double_fault.set_handler_fn(handler::double_fault);
    IDT.breakpoint.set_handler_fn(handler::breakpoint);
    IDT.page_fault.set_handler_fn(handler::page_fault);
    IDT[80].set_handler_fn(handler::syscall);
    IDT[0x20].set_handler_fn(handler::clock);

    IDT.load()
}
