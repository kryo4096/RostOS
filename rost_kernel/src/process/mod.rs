use core::sync::atomic::{AtomicU64, Ordering};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::boxed::Box;

use x86_64::structures::paging::PageTable;
use spin::*;

static CURRENT_PID : AtomicU64 = AtomicU64::new(0);
static NEXT_PID : AtomicU64 = AtomicU64::new(0x10);

static PROCESS_LIST : Once<RwLock<ProcessList>> = Once::new(); 

pub type ProcessList = BTreeMap<u64, Arc<RwLock<Process>>>;

#[repr(packed)]
pub struct CPUState {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64, 
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rip: u64,
}

impl CPUState {
    fn blank() -> Self {

        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rbp: 0,
            rsi: 0,
            rdi: 0,
            rsp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rflags: 0,
            rip: 0,
        }
    }
}

pub struct Process {
    pub pid: u64,
    pub state: Box<CPUState>,
    pub brk: u64,
    pub page_table: PageTable,
    pub runnable: bool,
}

pub fn proc_list() -> RwLockReadGuard<'static, ProcessList> {

    PROCESS_LIST.call_once(init_proc_list).read()
}


pub fn proc_list_mut() -> RwLockWriteGuard<'static, ProcessList> {
    PROCESS_LIST.call_once(init_proc_list).write()
}


pub fn init_proc_list() -> RwLock<ProcessList> {
    RwLock::new(BTreeMap::new())
}

pub fn create_process() -> u64 {
    let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);

    pid
}   

pub unsafe fn switch(pid: u64) {

    

}