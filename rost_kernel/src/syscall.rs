use x86_64::registers::model_specific::{Msr, Efer, EferFlags};

extern "C" {
    fn __syscall();
}

pub unsafe fn init() {
    let mut lstar = Msr::new(0xC0000082);
    Efer::update(|efer| efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS));

    lstar.write(__syscall as _);
}

pub unsafe extern "C" fn syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9:u64) -> u64 {
    match rdi {
        0x0  => print(rsi, rdx),
        0x1  => println(rsi, rdx),
        0x2  => debug_num(rsi),
        0x10 => time(),
        _  => panic!("Invalid syscall!"),
    }
}

unsafe fn print(ptr: u64, len: u64) -> u64 {
    let slice = core::slice::from_raw_parts(ptr as _, len as usize);
    let s = core::str::from_utf8_unchecked(slice);
    print!("{}", s);
    0
}

unsafe fn println(ptr: u64, len: u64) -> u64 {
    let slice = core::slice::from_raw_parts(ptr as _, len as usize);
    let s = core::str::from_utf8_unchecked(slice);
    println!("{}", s);
    0
}

unsafe fn debug_num(num: u64) -> u64 {
    println!("debug: 0x{:x}", num);
    0
}

unsafe fn time() -> u64 {
    ::time::get()
}

