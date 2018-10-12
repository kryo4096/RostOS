mod map;
mod frame_allocator;

pub const PAGE_SIZE: usize = 0x1000; // frame and page size is 4 KiB

pub unsafe fn init() {
    map::load();
    map::print();
}


