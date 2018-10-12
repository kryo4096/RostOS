mod idt;
mod pic;

use x86_64::instructions::interrupts;

pub use self::pic::send_eoi;

pub unsafe fn init() {
    idt::init();
    pic::init();

    pic::unmask(0); // clock

    if !interrupts::are_enabled() {
        interrupts::enable();
    }
}
