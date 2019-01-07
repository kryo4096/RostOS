#![no_std]
#![no_main]
#![feature(start)]


#[macro_use]
extern crate rost_std;

use rost_std::process;
use rost_std::signal;
use rost_std::vga;

use core::sync::atomic::*;


#[start]
#[no_mangle]
fn _start() {
    let p = process::execute(b"/bin/terminal").unwrap();
    p.wait();
    process::exit();
}
