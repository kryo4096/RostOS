use alloc::string::String;
use consts::*;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::VirtAddr;
use process::{self, signal};

pub extern "x86-interrupt" fn breakpoint(frame: &mut ExceptionStackFrame) {
    println!(
        "breakpoint \nrip=0x{:x}",
        frame.instruction_pointer.as_u64()
    );
    process::debug();
}

pub extern "x86-interrupt" fn page_fault(
    frame: &mut ExceptionStackFrame,
    pcode: PageFaultErrorCode,
) {
    {
        let current = ::process::Process::current();

        println!(
            "EXCEPTION: PAGE FAULT\n{:#?}\n{:#?}",
            frame,
            pcode
        );
    }

    let addr : u64;

    unsafe {
        asm!("mov $0, cr2" : "=r"(addr) ::: "intel");
    }
    println!("tried to access: 0x{:x}", addr);
    process::debug();
    unsafe {
        process::exit();
    }
}

pub extern "x86-interrupt" fn double_fault(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!(
        "EXCEPTION: DOUBLE FAULT\n{:#?} ec: 0x{:x}",
        frame, error_code
    );
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn gpf(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn ui(frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: INVALID INSTRUCTION\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn invalid_tss(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: INVALID TSS\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn stack_segment_fault(
    frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: #SS\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn security_exception(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: SECURITY EXEPTION\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn segment_not_present(
    frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: SEGMENT NOT PRESENT\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn overflow(frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: OVERFLOW\n{:#?}", frame);
    process::debug();
    loop {
        unsafe { asm!("hlt") }
    }
}

pub extern "x86-interrupt" fn nmi(frame: &mut ExceptionStackFrame) {
    println!("NMI occured!\n{:#?}", frame);
    process::debug();
}

pub extern "x86-interrupt" fn divide_by_zero(frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: Division by Zero\n{:#?}", frame);
    process::debug();
    unsafe {
        process::exit();
    }
}

pub extern "x86-interrupt" fn debug(frame: &mut ExceptionStackFrame) {
    println!("DEBUG EXCEPTION\n{:#?}", frame);
    process::debug();
    loop {}
}

pub extern "x86-interrupt" fn bound_range_exceeded(frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: Bound Range Exceeded\n{:#?}", frame);

    process::debug();
    loop {}
}

extern "C" {
    pub fn syscall_handler();
    pub fn keyboard_handler();
}

/*
#[naked]
pub fn syscall() {
    unsafe {
        let args: *const [u64; 6];

        asm!("push r9
    push r8
    push rcx
    push rdx
    push rsi
    push rdi
    ": "=(rsp)"(args) ::: "intel", "volatile");
        let args = *args;
        let ret = ::syscall::syscall(args[0], args[1], args[2], args[3], args[4], args[5]);

        asm!("pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop r8
    pop r9
    iretq" :: "(rax)="(ret) :: "intel", "volatile");
    }
}
*/



pub extern "x86-interrupt" fn tick(frame: &mut ExceptionStackFrame) {
    unsafe {
        ::time::tick();
        ::interrupt::send_eoi(0);
        ::process::update();
    }
}


#[no_mangle]
pub extern "C" fn __keyboard() {
    let port = Port::new(KB_DATA_PORT);
    unsafe {
        
        let scancode: u8 = port.read();
        signal::signal_bus().call(1, scancode as _, 0,0,0);
        ::interrupt::send_eoi(1);
    }
}
