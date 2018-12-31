#[macro_use]
use crate::syscall::{self, *};

pub struct Process {
    pid: u64,
}

impl Process {

    pub fn pid(&self) -> u64 { 
        self.pid 
    }

    pub fn wait(&self) {
        unsafe {
            syscall!(SYS_PROCESS_WAIT, self.pid);
        }
    }

    pub fn kill(&self) {
        unsafe {
            syscall!(SYS_PROCESS_KILL, self.pid);
        }
    }
}

pub fn current() -> Process {
    let pid = unsafe {
        syscall!(SYS_PROCESS_GETPID)
    };

    Process {
        pid
    }
}

pub fn execute(elf_path: &[u8]) -> Option<Process> {
    let pid = unsafe {
        syscall!(SYS_PROCESS_EXECUTE, elf_path.as_ptr(), elf_path.len())
    };

    if pid == 0 {
        None
    } else {
        Some(Process {
            pid
        })
    }
}


pub fn sleep(ticks: u64) {
    unsafe {
        syscall!(SYS_PROCESS_SLEEP, ticks);
    }
}

pub fn exit() {
    unsafe {
        syscall!(SYS_PROCESS_EXIT);
    }
}

