#[macro_use]
use crate::syscall::{self, *};


#[derive(Copy, Clone)]
pub struct Time {
    tick_count: u64,
}

impl Time {
    pub fn current() -> Self {
        Self {
            tick_count: unsafe {syscall!(SYS_GET_TIME)}
        }
    }

    pub fn elapsed(&self) -> u64 {
        Self::current().tick_count - self.tick_count
    }
}