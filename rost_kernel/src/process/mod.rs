use core::sync::atomic::AtomicU64;

static CURRENT_PID : AtomicU64 = AtomicU64::new(0);

#[repr(packed)]
pub struct CPUState {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64, 
    r13: u64,
    r14: u64,
    r15: u64,
    rflags: u64,
    rip: u64,
}

pub struct Process {
    pid: u64,
    state: CPUState,
    brk: u64,
}