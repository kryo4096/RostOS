pub const SYS_MAP_VGA: u64 = 0x40;
pub const SYS_UNMAP_VGA: u64 = 0x41;

#[naked]
pub unsafe extern "C" fn _syscall(rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9: u64) -> u64 {
    asm!("int 0x80")
}

#[macro_export]
macro_rules! syscall {
    ($rdi:expr)                                                         => (crate::syscall::_syscall($rdi,0,0,0,0,0));
    ($rdi:expr, $rsi:expr)                                              => (crate::syscall::_syscall($rdi,$rsi,0,0,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr)                                   => (crate::syscall::_syscall($rdi,$rsi,$rdx,0,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr)                        => (crate::syscall::_syscall($rdi,$rsi,$rdx,$rcx,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr, $r8:expr)              => (crate::syscall::_syscall($rdi,$rsi,$rdx,$rcx,$r8,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr, $r8:expr, $r9:expr)    => (crate::syscall::_syscall($rdi,$rsi,$rdx,$rcx,$r8,$r9));
}
