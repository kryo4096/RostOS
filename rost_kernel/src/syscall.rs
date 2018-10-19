pub unsafe fn syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9:u64) {
    //println!("Syscall! {:?}", (rdi, rsi, rdx, rcx, r8, r9));
    match rdi {
        0 => kprintln(rsi, rdx),
        _ => panic!("Invalid syscall!"),
    }
}

unsafe fn kprintln(ptr: u64, len: u64) {
    let slice = core::slice::from_raw_parts(ptr as _, len as usize);
    let s = core::str::from_utf8_unchecked(slice);
    println!("{}", s);
}
