use bootloader::bootinfo::BootInfo;

static mut BOOT_INFO: Option<&'static BootInfo> = None;

pub fn get_info() -> &'static BootInfo {
    unsafe {
        if let Some(_boot_info) = BOOT_INFO {
            _boot_info
        } else {
            BOOT_INFO = Some(&*(0xffff820000000000 as *const BootInfo));

            get_info()
        }
    }
}

pub fn print_map() {
    let info = get_info();

    for entry in info.memory_map.iter() {
        println!("{:?}", entry);
    }
}
