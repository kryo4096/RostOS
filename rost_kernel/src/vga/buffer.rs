use super::color::*;

use spin::Mutex;

use consts::*;

/// Represents the VGA buffer which stores ASCII characters and their colors. Anything written to it will appear on the screen. Origin is top-left.
pub struct VGABuffer {
    buffer_ptr: *mut u8,
    width: usize,
    height: usize,
}

unsafe impl Send for VGABuffer {}

unsafe impl Sync for VGABuffer {}

impl VGABuffer {
    
    /// Creates a new handle to the VGA buffer. This function only makes sense when it is called with addr=0xb8000. 
    const unsafe fn new(addr: u64, width: usize, height: usize) -> Self {
        Self {
            buffer_ptr: addr as _, // convert usize to buffer pointer (extremely unsafe)
            width, // buffer width, can cause memory issues if set incorrectly
            height, // buffer height, same problem as aboce
        }
    }
    /// Returns the height of the buffer.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the width of the buffer.
    pub fn height(&self) -> usize {
        self.height 
    }

    /// Puts a colored ASCII character at the specified position. Will panic if the position is out of bounds!
    pub fn put_char(&self, x: usize, y: usize, chr: u8, bg_color: Color, fg_color: Color) {
        if x >= self.width || y >= self.height {
            panic!("VGA Index out of bounds!");
        }
        let index = (y * self.width + x) as isize;
        unsafe {
            *self.buffer_ptr.offset(index * 2 + 0) = chr;
            *self.buffer_ptr.offset(index * 2 + 1) = (bg_color as u8) << 4 | fg_color as u8;
        }
    }

    pub fn clear_line(&self, line: usize, bg_color: Color) {

        let col = (bg_color as u8) << 4;

        for i in 0..line {
            let i = (line * self.width + i) as isize;
            unsafe {
                *self.buffer_ptr.offset(i*2 + 0) = b' ';
                *self.buffer_ptr.offset(i*2 + 1) = col;
            }
        }
    }

    pub fn clear_bg_color(&self, bg_color: Color) {

        let col = (bg_color as u8) << 4;

        for i in 0..self.width*self.height {
            let i = i as isize;
            unsafe {
                *self.buffer_ptr.offset(i*2 + 1) = col & 0xf | (bg_color as u8) << 4;
            }
        }
    } 

    pub fn clear(&self, bg_color: Color) {

        let col = (bg_color as u8) << 4;

        for i in 0..self.width*self.height {
            let i = i as isize;
            unsafe {
                *self.buffer_ptr.offset(i*2 + 0) = b' ';
                *self.buffer_ptr.offset(i*2 + 1) = col;
            }
        }
    } 

    pub fn shift_up(&self) {
        for i in self.width*2..self.height*self.width*2 {
            let i = i as isize;
            unsafe {
                *self.buffer_ptr.offset(i - (self.width*2) as isize) = *self.buffer_ptr.offset(i);    
            }
        }
    }


}

/// The VGA buffer at 0xb8000
pub static VGA_BUFFER: VGABuffer = unsafe { VGABuffer::new(VGA_BUFFER_VADDR, 80, 25) };
