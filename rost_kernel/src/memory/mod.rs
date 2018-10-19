use consts;
use x86_64::structures::paging::*;

mod map;
mod frame_allocator;

pub unsafe fn init() -> frame_allocator::FrameStackAllocator {
    map::load();
    frame_allocator::FrameStackAllocator::new(&mut map::MEMORY_MAP)
}

pub fn debug_page_table() {
    let mut p4 = unsafe { &mut *(consts::P4_TABLE_ADDR as *mut PageTable) };

    for i in 0..511 {
        let ent = &p4[i];

        if ent.flags().contains(PageTableFlags::PRESENT) {
            println!("{}: {:?}",i,ent.flags());
        }
    }
}