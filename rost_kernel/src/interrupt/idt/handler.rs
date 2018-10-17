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
    let n: u64;
    let arg1: u64;
    unsafe {
        asm!("mov $0, rdi" : "=r"(n) ::: "intel");
        asm!("mov $0, rsi" : "=r"(arg1) ::: "intel");
    }

    match n {
        0 => println!("{}", arg1),
        _ => println!("Unknown syscall!"),
    }
}

pub extern "x86-interrupt" fn clock(frame: &mut ExceptionStackFrame) {
    ::time::tick();
    unsafe {
        ::interrupt::send_eoi(0);
    }
}
