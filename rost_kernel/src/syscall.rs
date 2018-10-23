pub unsafe fn syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9:u64) -> Option<u64> {
    //println!("Syscall! {:?}", (rdi, rsi, rdx, rcx, r8, r9));
    match rdi {
        0 => print(rsi, rdx),
        1 => println(rsi, rdx),
        2 => debug_num(rsi),
        10 => time(),
        _ => panic!("Invalid syscall!"),
    }

}

unsafe fn print(ptr: u64, len: u64) -> Option<u64> {
    let slice = core::slice::from_raw_parts(ptr as _, len as usize);
    let s = core::str::from_utf8_unchecked(slice);
    print!("{}", s);
    None
}

unsafe fn println(ptr: u64, len: u64) -> Option<u64> {
    let slice = core::slice::from_raw_parts(ptr as _, len as usize);
    let s = core::str::from_utf8_unchecked(slice);
    println!("{}", s);
    None
}

unsafe fn debug_num(num: u64) -> Option<u64> {
    println!("debug: 0x{:x}", num);
    None
}

unsafe fn time() -> Option<u64> {
    Some(::time::get())
}

