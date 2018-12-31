use alloc::vec::Vec;

use x86_64::structures::paging::PageTableFlags;

use xmas_elf::header;
use xmas_elf::header::HeaderPt2;
use xmas_elf::program::{ProgramHeader, ProgramHeader64};
use xmas_elf::ElfFile;

use core::ptr;

use crate::memory;

pub struct LoadInfo {
    pub entry_point: u64,
}

pub unsafe fn load_elf(_elf: &[u8]) -> Result<LoadInfo, &'static str> {
    let elf = ElfFile::new(_elf)?;
    header::sanity_check(&elf)?;

    let entry_point = elf.header.pt2.entry_point();

    let segments: Vec<ProgramHeader64> = elf
        .program_iter()
        .map(|p| match p {
            ProgramHeader::Ph64(header) => Ok(*header),
            _ => Err("32-bit ELFs not supported"),
        })
        .collect::<Result<Vec<_>, _>>()?;

    for segment in segments {
        //println!("{:x?}", segment);
        let vaddr = segment.virtual_addr;
        let off = segment.offset;
        let file_size = segment.file_size;
        memory::map_range_all(
            vaddr,
            vaddr + segment.mem_size,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );

        ptr::copy(_elf.as_ptr().offset(off as _), vaddr as _, file_size as _);
    }

    Ok(LoadInfo { entry_point })
}
