use vga;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(
    _panic_info: &PanicInfo,
) -> ! {
    vga::set_background(vga::Color::Red);
    vga::clear();
    println!("kernel panic");
    
    loop {}
}

