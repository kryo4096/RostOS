mod handler;
use core::mem;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn init() {
    IDT.double_fault.set_handler_fn(handler::double_fault);
    IDT.breakpoint.set_handler_fn(handler::breakpoint);
    IDT.page_fault.set_handler_fn(handler::page_fault);
    IDT[0x80].set_handler_fn(*(handler::_syscall_handler as *const extern "x86-interrupt" fn(&mut ExceptionStackFrame)));
    IDT[0x20].set_handler_fn(handler::clock);

    IDT.load()
}
