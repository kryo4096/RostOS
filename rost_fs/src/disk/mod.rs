pub mod block;

use core::mem;
use core::option::NoneError;

use self::block::{Block, RootBlock, BLOCK_SIZE};
use crate::node::node::{self, Node};
use core::ptr::Unique;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct DiskAddress(u64);

impl DiskAddress {
    pub const NULL: Self = DiskAddress(0xffff_ffff_ffff_ffff);

    pub const fn block(index: u64) -> Self {
        DiskAddress(index << 12)
    }

    pub const fn new(index: u64, offset: u64) -> Self {
        DiskAddress(index << 12 | offset & 0xfff)
    }

    pub fn is_null(&self) -> bool {
        self.full_address().is_none()
    }

    pub fn full_address(&self) -> Option<u64> {
        if *self == Self::NULL {
            None
        } else {
            Some(self.0)
        }
    }

    pub fn offset(&self) -> Option<u64> {
        Some(self.full_address()? & 0xfff)
    }

    pub fn index(&self) -> Option<u64> {
        Some(self.full_address()? >> 12)
    }

    pub fn inc(&mut self) -> Self {
        self.0 += BLOCK_SIZE;
        *self
    }
}

pub trait Disk {
    /// Get a mutable reference to a block on disk
    fn get_block(&self, index: DiskAddress) -> Option<&mut Block>;
    /// Get the total amount of blocks.
    fn block_count(&self) -> u64;
}

pub struct RamDisk {
    start: Unique<Block>,
    block_count: u64,
}

impl RamDisk {
    pub const fn new_empty() -> Self {
        Self {
            start: Unique::empty(),
            block_count: 0,
        }
    }

    pub unsafe fn init(&mut self, start_addr: u64, size: u64) {
        *self = Self {
            start: Unique::new_unchecked((start_addr & !0xfff) as _),
            block_count: (size & !0xfff) / BLOCK_SIZE,
        }
    }
}

impl Disk for RamDisk {
    fn get_block(&self, addr: DiskAddress) -> Option<&mut Block> {
        unsafe {
            if addr.index()? < self.block_count() {
                Some(&mut *self.start.as_ptr().offset(addr.index()? as _))
            } else {
                None
            }
        }
    }

    fn block_count(&self) -> u64 {
        self.block_count
    }
}

pub fn format(disk: &impl Disk, name: &[u8]) -> Result<(), NoneError> {
    let root_block = block::get_root_block(disk);

    *root_block = RootBlock::init(name);

    Ok(())
}
