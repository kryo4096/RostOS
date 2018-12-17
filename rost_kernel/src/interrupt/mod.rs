mod pic;
mod handler;

use x86_64::instructions::interrupts;

pub use self::pic::send_eoi;

use core::mem;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable, HandlerFunc};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn init() {
    IDT.double_fault.set_handler_fn(handler::double_fault)
        .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.breakpoint.set_handler_fn(handler::breakpoint);
    IDT.page_fault.set_handler_fn(handler::page_fault);
    IDT.general_protection_fault.set_handler_fn(handler::gpf);
    IDT.invalid_opcode.set_handler_fn(handler::ui);
    IDT.invalid_tss.set_handler_fn(handler::invalid_tss);
    IDT.stack_segment_fault.set_handler_fn(handler::stack_segment_fault);
    IDT.security_exception.set_handler_fn(handler::security_exception);
    IDT.segment_not_present.set_handler_fn(handler::segment_not_present);
    IDT.overflow.set_handler_fn(handler::overflow);
    IDT.non_maskable_interrupt.set_handler_fn(handler::nmi);
    IDT.bound_range_exceeded.set_handler_fn(handler::bound_range_exceeded);
    IDT.divide_by_zero.set_handler_fn(handler::divide_by_zero);
    IDT[0x80].set_handler_fn(*(&(handler::syscall_handler as unsafe extern fn()) as *const unsafe extern fn() as u64 as *const HandlerFunc));
    IDT[0x20].set_handler_fn(handler::tick);
    IDT[0x21].set_handler_fn(handler::keyboard);


    IDT.load();

    pic::init();

    pic::unmask(0); //timer
    pic::unmask(1); //keyboard

    if !interrupts::are_enabled() {
        interrupts::enable();
    }
}
