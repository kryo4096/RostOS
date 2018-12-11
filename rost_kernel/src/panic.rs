use vga_buffer;

use core::panic::PanicInfo;
use core::alloc::Layout;


#[no_mangle]
#[panic_handler]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {
    println!("panic at: {:#?}",panic_info.location().unwrap());

    if let Some(msg) = panic_info.message() {
        println!("{}", msg);
    }
    
    loop {}
}

#[alloc_error_handler]
pub fn oom(_:Layout) -> ! {
    panic!("Out of Memory!")
}