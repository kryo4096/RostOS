#[macro_use]
use crate::syscall::{self, *};


pub fn subscribe(channel: u64, handler_func: extern "C" fn(u64,u64,u64,u64)) {
    unsafe {
        syscall!(SYS_SUBSCRIBE, channel, handler_func as *const extern "C" fn(u64,u64,u64,u64));
    }
}

pub fn add_channel(channel: u64) {
    unsafe {
        syscall!(SYS_ADD_CHANNEL, channel);
    }
}

pub fn send(channel: u64, arg0: u64, arg1: u64, arg2: u64, arg3: u64) {
    unsafe {
        syscall!(SYS_SEND, channel, arg0, arg1, arg2, arg3);
    }
}


