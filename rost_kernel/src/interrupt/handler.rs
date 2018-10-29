use consts::*;
use x86_64::instructions::port::Port;
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

pub extern "x86-interrupt" fn double_fault(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?} ec: {}", frame, error_code);
    loop {}
}

pub extern "x86-interrupt" fn gpf(frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", frame);
    loop {}
}

extern {
    pub fn syscall_handler(); 
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
pub extern "x86-interrupt" fn clock(frame: &mut ExceptionStackFrame) {
    ::time::tick();
    unsafe {
        ::interrupt::send_eoi(0);
    }
}

pub extern "x86-interrupt" fn keyboard(frame: &mut ExceptionStackFrame) {
    let port = Port::new(KB_DATA_PORT);
    unsafe {
        let scancode: u8 = port.read();
        ::interrupt::send_eoi(1);
        println!("{}d", scancode);
    }
}
