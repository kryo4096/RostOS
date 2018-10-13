use vga;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {

    println!("panic at: {:?}",panic_info.location().unwrap());




    loop {}
}

