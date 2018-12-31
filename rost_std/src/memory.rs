#[macro_use]
use crate::syscall::{self, *};

pub const MAX_ADDRESS: u64 = 0x0000_8000_0000_0000 - 1;

pub unsafe fn map_page(virt_addr: u64) {
    syscall!(SYS_VMAP, virt_addr, virt_addr);
}

pub unsafe fn map_range(virt_start: u64, virt_len: u64) {
    syscall!(SYS_VMAP, virt_start, virt_start + virt_len);
}

pub unsafe fn map_to(virt_addr: u64, phys_addr: u64) {
    syscall!(SYS_PMAP, virt_addr, phys_addr);
}