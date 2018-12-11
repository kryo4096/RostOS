use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use fs::NodeID;
use memory;
use spin::*;
use x86_64::structures::paging::PageTable;

static CURRENT_PID: AtomicU64 = AtomicU64::new(0);
static NEXT_PID: AtomicU64 = AtomicU64::new(0x10);

static PROCESS_STATE: Once<ProcessState> = Once::new();

pub type ProcessMap = BTreeMap<u64, RwLock<Process>>;
pub type ProcessQueue = VecDeque<u64>;

pub struct ProcessState {
    processes: RwLock<ProcessMap>,
    queue: RwLock<ProcessQueue>, 
}

pub fn init_process_state() -> ProcessState {
    ProcessState {
        processes: RwLock::new(BTreeMap::new()),
        queue: RwLock::new(VecDeque::new()),
    }
}

pub fn processes() -> RwLockReadGuard<'static, ProcessMap> {
    PROCESS_STATE.call_once(init_process_state).processes.read()
}

pub fn processes_mut() -> RwLockWriteGuard<'static, ProcessMap> {
    PROCESS_STATE.call_once(init_process_state).processes.write()
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,
    pub cr3: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
}

pub enum FDKind {
    File,
}

pub struct FileDescriptor {
    kind: FDKind,
    file: NodeID,
}

pub struct MemoryRegion {
    start: u64,
    end: u64,
}

pub enum State {
    Uninitialized,
    Initialized,
    Paused,
    Running,
}

pub struct Process {
    pub regs: Registers,
    pub runnable: bool,
    pub state: State,
    pub exec_image: MemoryRegion,
}

use consts::*;

pub fn create_process() -> u64 {
    let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);

    let paddr = memory::create_table(pid);

    let process = Process {
        regs: Registers {
            cr3: paddr,
            .. Registers::default()
        },
        runnable: false,
        state: State::Uninitialized,
        exec_image: MemoryRegion {
            start: 0,
            end: 0,
        },
    };
    {
        let mut list = processes_mut();
        list.insert(pid, RwLock::new(process));
    }
    
    pid
}

use alloc::vec::Vec;
use fs::path::Path;
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PageTableFlags;

pub unsafe fn load_elf_to_process(pid: u64, elf_path: Path) {
    
    let list = processes_mut();

    let mut process = list
        .get(&pid)
        .expect("load_elf_to_process(): process doesn't exist")
        .write();

    let old_paddr = memory::load_table(process.regs.cr3);

    let mut buf = Vec::new();

    let file = ::fs::open(&mut *::fs::tree_mut(), elf_path, 0)
        .expect("load_elf_to_process(): executable doesn't exist");

    ::fs::read_file(&mut *::fs::tree_mut(), file, &mut buf);
    
    let info = ::elf::load_elf(&mut buf).expect("load_elf_to_process(): failed to load executable");

    memory::map_range(
        USER_STACK_TOP - USER_STACK_SIZE,
        USER_STACK_TOP,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
    ).expect("load_elf_to_process(): failed to map user stack");

    process.regs.rip = info.entry_point;
    process.regs.rsp = USER_STACK_TOP;
    process.exec_image.start = info.image_start;
    process.exec_image.end = info.image_end;
    process.state = State::Initialized;
    process.runnable = true;
    memory::load_table(old_paddr);
}

pub unsafe fn start_process(pid: u64) -> ! {
    CURRENT_PID.store(pid, Ordering::SeqCst);

    let mut list = processes_mut();
    let mut process = list
        .get(&pid)
        .expect("load_elf_to_process(): process doesn't exist")
        .write();

    process.runnable = false;
    process.state = State::Running;

    memory::load_table(process.regs.cr3);

    asm!("mov rsp, $0; jmp $1" : : "r"(process.regs.rsp), "r"(process.regs.rip) :: "intel", "volatile");

    unreachable!();
}

pub fn current_pid() -> u64 {
    CURRENT_PID.load(Ordering::SeqCst)
}
