use consts::*;
use spin::{Mutex, MutexGuard, Once};
use x86_64::structures::paging::*;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::ux::u9;

pub mod frame_allocator;
mod map;

use self::frame_allocator::FrameStackAllocator;

static FRAME_ALLOCATOR: Once<Mutex<FrameStackAllocator>> = Once::new();

pub unsafe fn init() {
    map::load();
}

pub fn debug_page_table() {
    for i in 0..511 {
        let mut p4 = unsafe { &mut *(P4_TABLE_ADDR as *mut PageTable) };
        let ent = &p4[i];

        if ent.flags().contains(PageTableFlags::PRESENT) {
            println!("{}: {:?}", i, ent.flags());
        }
    }
}

pub fn p4_t() -> &'static mut PageTable {
    unsafe { &mut *(P4_TABLE_ADDR as *mut PageTable) }
}

pub fn p4() -> RecursivePageTable<'static> {
    let mut p4 = unsafe { &mut *(P4_TABLE_ADDR as *mut PageTable) };

    RecursivePageTable::new(p4).unwrap()
}

pub fn frame_allocator() -> MutexGuard<'static, FrameStackAllocator> {
    unsafe {
        FRAME_ALLOCATOR
            .call_once(|| Mutex::new(FrameStackAllocator::new(&mut map::MEMORY_MAP)))
            .lock()
    }
}

pub unsafe fn load_table(new_paddr: u64) -> u64 {
    let old_paddr;
    
    asm!("mov $0, cr3":"=r"(old_paddr):::"intel", "volatile");
    asm!("mov cr3, $0"::"r"(new_paddr)::"intel", "volatile");
    old_paddr
}

pub fn create_table(position: u64) -> u64 {
    unsafe {
        let vaddr = PT_START + position * PAGE_SIZE;
        let frame = map(vaddr, PageTableFlags::WRITABLE | PageTableFlags::PRESENT).expect("failed to map page");

        let mut table = &mut *(vaddr as *mut PageTable);

        *table = PageTable::new();

        for i in 0..512u16 {
            let i = u9::new(i);
            table[i] = p4_t()[i].clone();
        }
        
        for i in 0..256u16 {
            let i = u9::new(i);
            table[i] = PageTableEntry::new();
        }

        table[u9::new(511)].set_addr(PhysAddr::new(frame), PageTableFlags::PRESENT | PageTableFlags::WRITABLE);

        frame
    }
}

pub fn translate(virt: u64) -> Option<u64> {
    p4().translate_page(Page::<Size4KiB>::containing_address(VirtAddr::new(virt)))
        .map(|x| x.start_address().as_u64())
}

pub fn map_to_address(virt: u64, phys: u64, flags: PageTableFlags) -> Result<(), MapToError> {
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(virt));
    let frame = PhysFrame::containing_address(PhysAddr::new(phys));

    p4().map_to(page, frame, flags, &mut *frame_allocator())?
        .flush();

    Ok(())
}

pub fn map(virt: u64, flags: PageTableFlags) -> Result<u64, MapToError> {
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(virt));
    //println!("mapping 0x{:x}", page.start_address().as_u64());

    let frame = frame_allocator().alloc().expect("no more memory");
    p4().map_to(page, frame, flags, &mut *frame_allocator())?
        .flush();

    Ok(frame.start_address().as_u64())
}

pub fn unmap(virt: u64) {
    let page = Page::<Size4KiB>::containing_address(VirtAddr::new(virt));

    p4().unmap(page);
}

pub fn map_range(start_addr: u64, end_addr: u64, flags: PageTableFlags) -> Result<(), MapToError> {
    let start_page = Page::<Size4KiB>::containing_address(VirtAddr::new(start_addr));
    let end_page = Page::<Size4KiB>::containing_address(VirtAddr::new(end_addr));

    //println!("mapping from 0x{:x} to 0x{:x}", start_page.start_address().as_u64(), end_page.start_address().as_u64());

    for page in Page::range_inclusive(start_page, end_page) {
        let frame = frame_allocator().alloc().expect("no more memory");

        p4().map_to(page, frame, flags, &mut *frame_allocator())?
            .flush();
    }

    Ok(())
}

pub fn map_range_all(start_addr: u64, end_addr: u64, flags: PageTableFlags) {
    let start_page = Page::<Size4KiB>::containing_address(VirtAddr::new(start_addr));
    let end_page = Page::<Size4KiB>::containing_address(VirtAddr::new(end_addr));

    //println!("mapping from 0x{:x} to 0x{:x}", start_page.start_address().as_u64(), end_page.start_address().as_u64());


    for page in Page::range_inclusive(start_page, end_page) {
        let frame = frame_allocator().alloc().expect("no more memory");

        let res = p4().map_to(page, frame, flags, &mut *frame_allocator());

        if let Ok(res) = res {
            res.flush();
        }
    }
}
