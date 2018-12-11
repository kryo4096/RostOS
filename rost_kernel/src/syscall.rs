use x86_64::registers::model_specific::{Msr, Efer, EferFlags};
use x86_64::instructions::port::*;
use memory;

#[no_mangle]
pub unsafe extern "C" fn __syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9: u64) -> u64 {
    match rdi {
        0x0  => print(rsi, rdx),
        0x1  => println(rsi, rdx),
        0x2  => debug(rsi, rdx),
        0x10 => time(),
        0x20 => read_scancode(),
        _  => panic!("Invalid syscall!"),
    }
}

unsafe fn read_scancode() -> u64 {
    if let Some(scancode) = ::keyboard::pop_scancode() {
        scancode as _
    } else {
        0x0
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


unsafe fn debug(num: u64, f: u64) -> u64 {
    match f {
        2 => println!("{}", num),
        3 => println!("0x{:x}", num),
        0 => println!("0b{:b}", num),
        1 => println!("0o{:o}", num),
        _ => panic!(),
    }   
    0
}

unsafe fn time() -> u64 {
    ::time::get()
}
