use x86_64::registers::model_specific::{Msr, Efer, EferFlags};
use x86_64::instructions::port::*;
use memory;

extern "C" {
    fn syscall_handler();
}

pub unsafe fn init() {
    const LSTAR : Msr = Msr::new(0xC0000082);
    const SFMASK : Msr = Msr::new(0xC0000084);
    Efer::write(Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS);

    LSTAR.write(syscall_handler as *const fn() as _);
    SFMASK.write(0x200);
}

#[no_mangle]
pub unsafe extern "C" fn __syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9: u64) -> u64 {
    match rdi {
        0x0  => print(rsi, rdx),
        0x1  => println(rsi, rdx),
        0x2  => debug(rsi, rdx),
        0x10 => time(),
        0x20 => read_scancode(),
        0x30 => get_pid(),
        0x31 => exit(),
        0x32 => execute(rsi, rdx),
        0x33 => wait_exit(rsi),
        0x40 => map_vga(),
        0x41 => unmap_vga(),
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

unsafe fn get_pid() -> u64 {
    ::process::current_pid()
}

unsafe fn exit() -> u64 {
    ::process::exit();
    0
}

use fs::path::Path;
use alloc::vec::Vec;
use alloc::string::String;

unsafe fn execute(path_ptr: u64, path_len: u64) -> u64 {
    let path = core::slice::from_raw_parts(path_ptr as _, path_len as _);

    let mut vec = Vec::new();
    vec.extend_from_slice(path);

    let pid = ::process::Process::create(&vec);

    ::process::schedule(pid);

    pid
}

unsafe fn wait_exit(pid: u64) -> u64 {
    ::process::wait(::process::WaitReason::ProcessExit(pid));
    0
}

unsafe fn debug(num: u64, f: u64) -> u64 {
    match f {
        0 => print!("0b{:b}", num),
        1 => print!("0o{:o}", num),
        2 => print!("{}", num),
        3 => print!("0x{:x}", num),
        
        _ => return (-1) as _,
    }   
    0
}

unsafe fn time() -> u64 {
    ::time::get()
}

use x86_64::structures::paging::PageTableFlags;
use consts::*;

unsafe fn map_vga() -> u64 {
    if ::vga_buffer::map_for_user().is_ok() {
        USER_VGA
    } else {
        0
    }
}

unsafe fn unmap_vga() -> u64 {
    ::vga_buffer::unmap_for_user();
    0
}


