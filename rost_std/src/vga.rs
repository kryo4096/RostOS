use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use core::ptr;
#[macro_use]
use crate::syscall::{self,*};

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

static mut VGA_ADDRESS: *mut u16 = 0 as _;
static mut VGA_BUFFER: [u16; VGA_WIDTH * VGA_HEIGHT] = [0;VGA_WIDTH * VGA_HEIGHT];

static VGA_LOCK: AtomicBool = AtomicBool::new(false);

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

pub struct ColorCode {
    bg: Color,
    fg: Color,
}

impl ColorCode {
    pub fn new(bg: Color, fg: Color) -> Self {
        Self {bg, fg}
    }
}

impl Into<u8> for ColorCode {
    fn into(self) -> u8 {
        self.bg as _ | self.fg as _ << 4
    }
}

pub struct VGA;

impl VGA {
    pub fn try_get() -> Option<Self> {
        if !VGA_LOCK.compare_and_swap(false, true, Ordering::SeqCst) {
            unsafe {
                VGA_ADDRESS = syscall!(SYS_MAP_VGA);
            }
            Some(Self)
        } else {
            None
        }
    }

    pub fn write_char(&mut self, x: usize, y: usize, chr: u8, color_code: ColorCode) {
        if(x >= VGA_WIDTH || y >= VGA_HEIGHT) {
            return;
        }

        unsafe {
            VGA_BUFFER[VGA_WIDTH * y + x] = ((color_code as u16) << 8) | chr as u16 & 0x00ff;
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            for chr in VGA_BUFFER.iter_mut() {
                *chr = 0;
            }
        }
    }

    pub fn show(&mut self) {
        unsafe {
            ptr::copy(VGA_BUFFER.as_mut_ptr(), VGA_ADDRESS, VGA_WIDTH * VGA_HEIGHT);
        }
    }
}

impl Drop for VGA {
    fn drop(&mut self) {
        syscall!(SYS_UNMAP_VGA);
        VGA_LOCK.store(false, Ordering::SeqCst);
    }
}
