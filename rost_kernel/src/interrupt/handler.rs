use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::instructions::port::Port;
use consts::*;

pub extern "x86-interrupt" fn breakpoint(frame: &mut ExceptionStackFrame) {
    println!(
        "breakpoint \nrip=0x{:x}",
        frame.instruction_pointer.as_u64()
    );
}

pub extern "x86-interrupt" fn page_fault(
    frame: &mut ExceptionStackFrame,
    pcode: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT\n{:#?}\n{:#?}", frame, pcode);
    loop {}
}

pub extern "x86-interrupt" fn double_fault(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?} ec: {}", frame, error_code);
    loop {}
}

pub extern "x86-interrupt" fn gpf(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", frame);
    loop {}
}

pub extern "x86-interrupt" fn syscall(frame: &mut ExceptionStackFrame) {
    let (rdi, rsi, rdx, rcx, r8, r9);


    unsafe {
        asm!("mov $0, rdi" : "=r"(rdi) ::: "intel");
        asm!("mov $0, rsi" : "=r"(rsi) ::: "intel");
        asm!("mov $0, rdx" : "=r"(rdx) ::: "intel");
        asm!("mov $0, rcx" : "=r"(rcx) ::: "intel");
        asm!("mov $0, r8" : "=r"(r8) ::: "intel");
        asm!("mov $0, r9" : "=r"(r9) ::: "intel");

        let _ = ::syscall::syscall(rdi, rsi, rdx, rcx, r8, r9);
 
    }
}

pub extern "x86-interrupt" fn clock(frame: &mut ExceptionStackFrame) {
    ::time::tick();
    unsafe {
        ::interrupt::send_eoi(0);
    }
}

pub extern "x86-interrupt" fn keyboard(frame: &mut ExceptionStackFrame) {
    let port = Port::new(KB_DATA_PORT);
    unsafe {
        let scancode : u8 = port.read();
        ::interrupt::send_eoi(1);
        println!("{}d",scancode);
    }
    
}

