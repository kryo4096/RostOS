mod buffer;
mod color;
mod writer;

pub use self::color::*;
use self::writer::*;

use core::fmt;
use core::fmt::Write;

use spin::Mutex;

pub fn write_str(color: Color, string: &str) {
    WRITER.lock().write_str(string, color);
}

pub fn write_fmt(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args);
}

pub fn set_background(bg_color: Color) {
    WRITER.lock().set_background(bg_color);
}

pub fn clear() {
    WRITER.lock().clear()
}

pub fn del() {
    WRITER.lock().del()
}
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::write_fmt(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
