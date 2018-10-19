use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

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

pub extern "x86-interrupt" fn double_fault(frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", frame);
    loop {}
}

pub extern "x86-interrupt" fn syscall(frame: &mut ExceptionStackFrame) {
    let rdi: u64;
    let rsi: u64;
    let rdx: u64;
    let rcx: u64;
    let r8: u64;
    let r9: u64;

    unsafe {
        asm!("mov $0, rdi" : "=r"(rdi) ::: "intel");
        asm!("mov $0, rsi" : "=r"(rsi) ::: "intel");
        asm!("mov $0, rdx" : "=r"(rdx) ::: "intel");
        asm!("mov $0, rcx" : "=r"(rcx) ::: "intel");
        asm!("mov $0, r8" : "=r"(r8) ::: "intel");
        asm!("mov $0, r9" : "=r"(r9) ::: "intel");
        ::syscall::syscall(rdi, rsi, rdx, rcx, r8, r9);
    }
}

pub extern "x86-interrupt" fn clock(frame: &mut ExceptionStackFrame) {
    ::time::tick();
    unsafe {
        ::interrupt::send_eoi(0);
    }
}
