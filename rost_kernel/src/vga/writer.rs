use super::buffer::*;
use super::color::*;

use core::fmt::{self, Write};

use spin::Mutex;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    buffer: &VGA_BUFFER,
    cursor: (0, 0),
    bg_color: Color::Black,
});

pub struct Writer {
    buffer: &'static Mutex<VGABuffer>,
    cursor: (usize, usize), // x, y
    bg_color: Color,
}

impl Writer {
    pub fn set_background(&mut self, bg_color: Color) {
        self.bg_color = bg_color;
        self.buffer.lock().clear_bg_color(bg_color);
    }

    pub fn write_str(&mut self, string: &str, fg_color: Color) {
        for byte in string.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_char(byte, fg_color),
                // not part of printable ASCII range
                _ => self.write_char(0xfe, fg_color),
            }
        }
    }

    pub fn del(&mut self) {
        if self.cursor.0 == 0 {
            if self.cursor.1 == 0 {
                return;
            }
            self.cursor.0 = self.buffer.lock().width() - 1;
            self.cursor.1 -= 1;
            self.buffer.lock().put_char(
                self.cursor.0,
                self.cursor.1,
                b' ',
                self.bg_color,
                Color::White,
            );
            return;
        }
        self.cursor.0 -= 1;
        self.buffer.lock().put_char(
            self.cursor.0,
            self.cursor.1,
            b' ',
            self.bg_color,
            Color::White,
        );
    }

    pub fn write_char(&mut self, chr: u8, fg_color: Color) {
        if chr == b'\n' {
            self.new_line();
            return;
        }

        if self.cursor.0 >= self.buffer.lock().width() - 1 {
            self.new_line()
        }

        self.buffer
            .lock()
            .put_char(self.cursor.0, self.cursor.1, chr, self.bg_color, fg_color);

        self.cursor.0 += 1;
    }

    pub fn new_line(&mut self) {
        let h = self.buffer.lock().height();
        if self.cursor.1 >= h - 1 {
            self.buffer.lock().shift_up();
            self.buffer.lock().clear_line(h - 1, self.bg_color);
        } else {
            self.cursor.1 += 1;
        }

        self.cursor.0 = 0;
    }

    pub fn clear(&mut self) {
        self.buffer.lock().clear(self.bg_color);
        self.cursor = (0, 0);
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s, Color::White);
        Ok(())
    }
}
