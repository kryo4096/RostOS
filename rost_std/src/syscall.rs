pub const SYS_MAP_VGA: u64 = 0x40;
pub const SYS_UNMAP_VGA: u64 = 0x41;

#[naked]
pub unsafe extern "C" fn _syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9: u64) -> u64 {
    let ret;
    asm!("int 0x80" :"={rax}"(ret):::"intel", "volatile");
    ret
}
