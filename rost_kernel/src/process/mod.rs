use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use fs::NodeID;
use memory;
use spin::*;
use x86_64::structures::paging::PageTable;
use alloc::vec::Vec;
use fs::path::Path;
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PageTableFlags;
use fs::path;
use consts::*;


static CTX_SWITCH_LOCK: AtomicBool = AtomicBool::new(true);

static CURRENT_PID: AtomicU64 = AtomicU64::new(0);
static NEXT_PID: AtomicU64 = AtomicU64::new(1);

static SCHEDULER: Once<Scheduler> = Once::new();

pub type ProcessMap = BTreeMap<u64, Arc<RwLock<Process>>>;
pub type ProcessQueue = VecDeque<u64>;

pub struct Scheduler {
    processes: RwLock<ProcessMap>,
    queue: RwLock<ProcessQueue>,
}

pub fn init_process_state() -> Scheduler {
    Scheduler {
        processes: RwLock::new(BTreeMap::new()),
        queue: RwLock::new(VecDeque::new()),
    }
}

pub fn processes() -> RwLockReadGuard<'static, ProcessMap> {
    SCHEDULER.call_once(init_process_state).processes.read()
}

pub fn processes_mut() -> RwLockWriteGuard<'static, ProcessMap> {
    SCHEDULER.call_once(init_process_state).processes.write()
}


pub fn process_queue() -> RwLockWriteGuard<'static, ProcessQueue> {
    SCHEDULER.call_once(init_process_state).queue.write()
}


pub fn init() {
    processes_mut().insert(0, Arc::new(RwLock::new(Process {
        pid: 0,
        regs: Registers {
            cr3: memory::translate(P4_TABLE_ADDR).unwrap(),
            rsp: 0,
        },
        state: State::Running,
        file_descriptors: Default::default(),
        name: "KERNEL".into(),
    })));


    let syscall_stack_start = KERNEL_SYSCALL_STACK_START - 1;
    let syscall_stack_end = KERNEL_SYSCALL_STACK_START;

    memory::map_range(
        syscall_stack_end,
        syscall_stack_start,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
    ).expect("Process::create(): failed to map user stack");

    memory::map(USER_KERNEL_STACK_PTR,  PageTableFlags::PRESENT | PageTableFlags::WRITABLE);

    unsafe {
        *(USER_KERNEL_STACK_PTR as *mut u64) = syscall_stack_start;
    }

}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct Registers {
    pub cr3: u64,
    pub rsp: u64,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            cr3: 0,
            rsp: 0 as _
        }
    }
}

pub enum NDKind {
    File,
}

pub struct NodeDescriptor {
    kind: NDKind,
    node: NodeID,
}

impl NodeDescriptor {
    fn new(kind: NDKind, node: NodeID) -> NodeDescriptor {
        NodeDescriptor {
            kind, node,
        }
    }
}

pub struct MemoryRegion {
    start: u64,
    end: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaitReason {
    ProcessExit(u64),
    Timer(u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Waiting(WaitReason),
    Runnable,
    Running,
    Exited,
}

pub struct Process {
    pub pid: u64,
    pub regs: Registers,
    pub state: State,
    pub file_descriptors: BTreeMap<u64, NodeDescriptor>,
    pub name: Vec<u8>,
}



impl Process {
    /// Create a process from an elf file
    pub unsafe fn create(elf_path: Path) -> u64 {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);

        let mut process = Process {
            pid,
            regs: Registers {
                cr3: memory::create_table(pid),
                rsp: USER_STACK_TOP,
            },
            state: State::Runnable,
            file_descriptors: BTreeMap::new(),
            name: path::head(elf_path).into(),
        };

        let old_table = memory::load_table(process.regs.cr3);

        memory::map_range(
            USER_STACK_TOP - USER_STACK_SIZE,
            USER_STACK_TOP,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        ).expect("Process::create(): failed to map user stack");

        let syscall_stack_start = KERNEL_SYSCALL_STACK_START + KERNEL_SYSCALL_STACK_SIZE * (2*pid + 1)  - 1;
        let syscall_stack_end =syscall_stack_start - KERNEL_SYSCALL_STACK_SIZE;

        memory::map_range(
            syscall_stack_end,
            syscall_stack_start,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        ).expect("Process::create(): failed to map user stack");

        memory::map(USER_KERNEL_STACK_PTR,  PageTableFlags::PRESENT | PageTableFlags::WRITABLE);

        *(USER_KERNEL_STACK_PTR as *mut u64) = syscall_stack_start;

        let mut buf = Vec::new();

        let file = ::fs::open(&mut *::fs::tree_mut(), elf_path, 0)
            .expect("Process::create(): executable doesn't exist");

        ::fs::read_file(&mut *::fs::tree_mut(), file, &mut buf);

        let info = ::elf::load_elf(&mut buf).expect("Process::create(): failed to load executable");

        process.push(info.entry_point); // ret
        process.push(0);
        process.push(0);
        process.push(0);
        process.push(0);
        process.push(0);
        process.push(0);
        process.push(0x200); //rflags INTERRUPTS ENABLED

        let process = Arc::new(RwLock::new(process));

        {
            let mut list = processes_mut();
            list.insert(pid, Arc::clone(&process));
        }

        {
            let curr = Process::current();
            memory::load_table(old_table);
        }

        pid
    }

    pub unsafe fn push(&mut self, value: u64) {

        self.regs.rsp -= core::mem::size_of::<u64>() as u64;
        *(self.regs.rsp as *mut u64) = value;


    }


    fn add_node_descriptor(&mut self, node: NodeID, id: u64) -> ::fs::FSResult<()> {
        let fd_kind = match ::fs::get_header(&mut *::fs::tree_mut(), node)? {
            ::fs::NodeHeader::FILE => Ok(NDKind::File),
            _ => Err(::fs::FSError::NotADirectory),
        }?;
        let file_desc = NodeDescriptor::new(fd_kind, node);

        Ok(())
    }

    pub fn current() -> Arc<RwLock<Process>>{
        processes().get(&current_pid()).expect("impossible").clone()
    }

    pub fn get(pid: u64) -> Option<Arc<RwLock<Process>>> {
        processes().get(&pid).map(Arc::clone)
    }


}

pub fn schedule(pid: u64) {
    process_queue().push_front(pid);

}

pub fn activate_scheduler() {
    CTX_SWITCH_LOCK.store(false, Ordering::SeqCst);
}

fn next_process() -> u64 {
    let mut queue = process_queue();

    while let Some(pid) = queue.pop_back() {
        if let Some(process) = Process::get(pid) {
            let mut process = process.write();

            if process.state == State::Runnable {
                return pid;
            }

            if let State::Waiting(WaitReason::ProcessExit(wait_pid)) = process.state {
                if Process::get(wait_pid).is_none() {
                    process.state = State::Runnable;
                }

                queue.push_front(pid);
            }

        }
    }

    panic!("no more processes");

}

pub unsafe fn wait(reason: WaitReason) {
    {
        let mut current = Process::current();
        current.write().state = State::Waiting(reason);
    }

    schedule(current_pid());
    let next_pid = next_process();


    switch_process(next_pid);
}

pub unsafe fn exit() -> ! {

    {
        let mut current = Process::current();
        current.write().state = State::Exited;
    }

    processes_mut().remove(&current_pid()).expect("failed to remove exited process!");

    let next_pid = next_process();

    let (from, to, to_cr3) = {
        let to = Process::get(next_pid).unwrap();
        to.write().state == State::Running;

        let new_rsp = &to.read().regs.rsp;
        let new_cr3 = to.read().regs.cr3;

        let mut x :u64 = 0;

        (&mut x as _, new_rsp as _, new_cr3)
    };


    CURRENT_PID.store(next_pid, Ordering::SeqCst);
    switch_context(from, to, to_cr3);

    unreachable!();
}

pub unsafe fn update() {
    if !CTX_SWITCH_LOCK.load(Ordering::SeqCst) {
        {
            let mut current = Process::current();
            current.write().state = State::Runnable;
        }

        schedule(current_pid());
        let pid = next_process();

        switch_process(pid);

    }
}

pub unsafe fn switch_process(pid: u64) {

    if pid == current_pid() {
        {
            let mut current = Process::current();
            current.write().state = State::Running;
        }
        return;
    }

    let (from, to, to_cr3) = {
        let mut from = Process::current();

        let to = Process::get(pid).expect("process does not exist");
        to.write().state == State::Running;

        let old_rsp = &mut from.write().regs.rsp;
        let new_rsp = &to.read().regs.rsp;
        let new_cr3 = to.read().regs.cr3;

        (old_rsp as _, new_rsp as _, new_cr3)
    };


    CURRENT_PID.store(pid, Ordering::SeqCst);
    switch_context(from, to, to_cr3);
}

extern "C" {
    fn switch_context(this: *mut u64, next: *const u64, cr3: u64);
}

pub fn current_pid() -> u64 {
    CURRENT_PID.load(Ordering::SeqCst)
}
