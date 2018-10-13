use x86_64::structures::paging::{Page, PageTable, RecursivePageTable, Mapper, PageTableEntry};

pub struct ActivePageTable {
    table: RecursivePageTable,
}