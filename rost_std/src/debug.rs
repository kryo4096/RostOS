use core::fmt;

#[macro_use]
use crate::syscall::{self, *};

struct DebugPrinter;



impl fmt::Write for DebugPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        unsafe {
            syscall!(SYS_DEBUG_PRINT, bytes.as_ptr(), bytes.len());
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    DebugPrinter.write_fmt(args);
}

#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($($arg:tt)*) => (kprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}