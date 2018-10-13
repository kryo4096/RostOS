mod map;
mod frame_allocator;

pub const PAGE_SIZE: u64 = 0x1000; // frame and page size is 4 KiB

pub unsafe fn init() -> frame_allocator::FrameStackAllocator {
    map::load();
    map::print();

    frame_allocator::FrameStackAllocator::new(&mut map::MEMORY_MAP)
}