#![no_std]
#![no_main]
#![feature(start)]


#[macro_use]
extern crate rost_std;

use rost_std::process;
use rost_std::signal;
use rost_std::vga;
#[macro_use]
use rost_std::debug;

use core::sync::atomic::*;


#[start]
#[no_mangle]
fn _start() {
    process::execute(b"/bin/logo").unwrap().wait();
    loop {
        process::execute(b"/bin/shell").unwrap().wait();
        kprintln!("Tried to kill init shell!");
    }
}
