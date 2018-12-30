#[macro_use]
use crate::syscall;
use crate::syscall::*;

pub struct Process {
    pid: u64,
}

impl Process {
    pub fn create(elf_path: &[u8]) -> Option<Process> {
        let pid = syscall!(SYS_PROCESS_EXECUTE, elf_path.as_ptr(), elf_path.len());

        if pid == 0 {
            None
        } else {
            Some(Process {
                pid
            })
        }
    }

    pub fn pid(&self) -> u64 { 
        self.pid 
    }

    pub fn wait_on(&self) {
        syscall!(SYS_PROCESS_WAIT, self.pid);
    }

    pub fn sleep(ticks: u64) {
        
    }
}