///! This module provides an abstraction around the legacy VGA buffer. It uses an invisible buffer to quickly store changes and then the `vga::show` function to show them on screen.

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use core::ptr;
use core::marker::PhantomData;

#[macro_use]
use crate::syscall;
use crate::syscall::*;

use crate::memory;

/// The width of the VGA buffer
pub const VGA_WIDTH: usize = 80;

/// The height of the VGA buffer
pub const VGA_HEIGHT: usize = 25;

static mut VGA_ADDRESS: *mut u16 = (memory::MAX_ADDRESS + 1 - 0x100_000 + 0xb8000) as _;
static mut VGA_BUFFER: [u16; VGA_WIDTH * VGA_HEIGHT] = [0;VGA_WIDTH * VGA_HEIGHT];

static VGA_MAPPED: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
/// This struct represents a VGA code.
#[derive(Clone, Copy)]
pub struct ColorCode {
    bg: Color,
    fg: Color,
}

impl ColorCode {
    pub const fn new(bg: Color, fg: Color) -> Self {
        Self {bg, fg}
    }
}

impl Into<u16> for ColorCode {
    fn into(self) -> u16 {
        (self.bg as u8 as u16) << 4 | self.fg as u8 as u16
    }
}

fn check_mapped() {
    if !VGA_MAPPED.load(Ordering::SeqCst) {
        panic!("VGA not mapped!")
    }
}


/// Maps a region of virtual memory to the VGA buffer. Operations writing to the buffer before it is mapped will panic the process.
pub fn map() {
    unsafe {
        if !VGA_MAPPED.compare_and_swap(false, true, Ordering::SeqCst) {
            memory::map_to(VGA_ADDRESS as _, 0xb8000);
        }
    }
}

/// Writes a single character to the invisible buffer at the given position.
pub fn write_char(x: usize, y: usize, chr: u8, color_code: ColorCode) {
    if x >= VGA_WIDTH || y >= VGA_HEIGHT {
        return;
    }

    unsafe {
        VGA_BUFFER[VGA_WIDTH * y + x] = ((Into::<u16>::into(color_code)) << 8) | chr as u16 & 0x00ff;
    }
}

/// Shifts the contents of the invisible buffer up by one row.
pub fn shift_up() {
    for y in 0..VGA_HEIGHT - 1 {
        for x in 0..VGA_WIDTH {
            unsafe {
                VGA_BUFFER[VGA_WIDTH * y + x] = VGA_BUFFER[VGA_WIDTH * (y+1) + x]
            }
        }
    }

    for x in 0..VGA_WIDTH {
        unsafe {
            VGA_BUFFER[VGA_WIDTH * 24 + x] = 0; 
        }
    }
}

/// Clears the invisible buffer.
pub fn clear() {
    unsafe {
        for chr in VGA_BUFFER.iter_mut() {
            *chr = 0;
        }
    }
}

/// Copies the invisible buffer to the visible VGA buffer. Will panic if the VGA buffer has not been mapped using `vga::map`.
pub fn show() {
    check_mapped();
    unsafe {
        ptr::copy(VGA_BUFFER.as_mut_ptr(), VGA_ADDRESS, VGA_WIDTH * VGA_HEIGHT);
    }
}
