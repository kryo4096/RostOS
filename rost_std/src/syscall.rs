//Debug syscalls
pub const SYS_DEBUG_PRINT: u64 = 0x0;

// Keyboard syscalls
pub const SYS_POLL_KEYBOARD: u64 = 0x20;

// Process syscalls
pub const SYS_PROCESS_GETPID: u64 = 0x30;
pub const SYS_PROCESS_EXIT: u64 = 0x31;
pub const SYS_PROCESS_EXECUTE: u64 = 0x32;
pub const SYS_PROCESS_WAIT: u64 = 0x33;

// VGA syscalls
pub const SYS_MAP_VGA: u64 = 0x40;
pub const SYS_UNMAP_VGA: u64 = 0x41;



global_asm!("
.global _syscall

_syscall:
    int $0x80
    ret
");

extern "C" {
    pub fn _syscall(_rdi: u64, _rsi: u64, _rdx: u64, _rcx: u64, _r8: u64, _r9: u64) -> u64;
}

#[macro_export]
macro_rules! syscall {
    ($rdi:expr)                                                         => (crate::syscall::_syscall($rdi as _,0,0,0,0,0));
    ($rdi:expr, $rsi:expr)                                              => (crate::syscall::_syscall($rdi as _,$rsi as _,0,0,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr)                                   => (crate::syscall::_syscall($rdi as _,$rsi as _,$rdx as _,0,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr)                        => (crate::syscall::_syscall($rdi as _,$rsi as _,$rdx as _,$rcx as _,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr, $r8:expr)              => (crate::syscall::_syscall($rdi as _,$rsi as _,$rdx as _,$rcx as _,$r8 as _,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr, $r8:expr, $r9:expr)    => (crate::syscall::_syscall($rdi as _,$rsi as _,$rdx as _,$rcx as _,$r8 as _,$r9 as _));
}
