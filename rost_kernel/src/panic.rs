use vga;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {

    vga::set_background(vga::Color::Red);
    vga::clear();

    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {:?}", s);
    } else {
        println!("panic occurred");
    }

    loop {}
}

