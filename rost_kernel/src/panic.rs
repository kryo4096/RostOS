use vga_buffer;

use core::panic::PanicInfo;
use core::alloc::Layout;

use alloc::string::String;



#[no_mangle]
#[panic_handler]
pub fn panic(
    panic_info: &PanicInfo,
) -> ! {
    let current = ::process::Process::current();
    println!("panic at: {:#?}\nin process {} (pid={})",panic_info.location().unwrap(), String::from_utf8_lossy(&current.read().name),::process::current_pid());

    if let Some(msg) = panic_info.message() {
        println!("{}", msg);
    } else {
        println!("no message");
    }
    
    loop {}
}

#[alloc_error_handler]
pub fn oom(_:Layout) -> ! {
    panic!("Out of Memory!")
}