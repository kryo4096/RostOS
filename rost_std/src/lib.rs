#![feature(asm, naked_functions)]
#![no_std]

#[macro_export]
macro_rules! syscall {
    ($rdi:expr)                                                         => (crate::syscall::_syscall($rdi,0,0,0,0,0));
    ($rdi:expr, $rsi:expr)                                              => (crate::syscall::_syscall($rdi,$rsi,0,0,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr)                                   => (crate::syscall::_syscall($rdi,$rsi,$rdx,0,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr)                        => (crate::syscall::_syscall($rdi,$rsi,$rdx,$rcx,0,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr, $r8:expr)              => (crate::syscall::_syscall($rdi,$rsi,$rdx,$rcx,$r8,0));
    ($rdi:expr, $rsi:expr, $rdx:expr, $rcx:expr, $r8:expr, $r9:expr)    => (crate::syscall::_syscall($rdi,$rsi,$rdx,$rcx,$r8,$r9));
}


use core::panic::PanicInfo;

pub mod vga;
pub mod keyboard;
mod syscall;

#[no_mangle]
#[panic_handler]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {
    loop {}
}

