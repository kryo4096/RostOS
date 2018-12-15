pub const PAGE_SIZE: u64 = 0x1000; // frame and page size is 4 KiB
pub const FRAME_STACK_SIZE: usize = 0x1000; // amount of freed frames stored before they are discarded

pub const P4_TABLE_ADDR: u64 = 0xffff_ffff_ffff_f000;

// MEMORY MAP

pub const MAX_ADDR: u64 = 0xffff_ffff_ffff_ffff;
pub const P4_ENTRY_SIZE: u64 = 0x80_0000_0000;

pub const KERNEL_START: u64 = MAX_ADDR - P4_ENTRY_SIZE * 256 + 1;

pub const KERNEL_HEAP_START: u64 = KERNEL_START + P4_ENTRY_SIZE;
pub const KERNEL_HEAP_SIZE: u64 = 0x100_0000; // 16 MiB



pub const VGA_BUFFER_VADDR: u64 = KERNEL_START + 2 * P4_ENTRY_SIZE;
pub const VGA_BUFFER_PADDR: u64 = 0xb8000;

pub const RAMDISK_START: u64 = KERNEL_START + 3 * P4_ENTRY_SIZE;
pub static RAMDISK_SIZE: u64 = ::DISK_IMAGE.len() as u64;

pub const PT_START: u64 = KERNEL_START + 4 * P4_ENTRY_SIZE;

pub const KERNEL_SYSCALL_STACK: u64 = KERNEL_START + 6*P4_ENTRY_SIZE - 1;// 0xffff_82ff_ffff_fffe
pub const KERNEL_SYSCALL_SIZE: u64 = 0x100_0000;

pub const USER_STACK_TOP: u64 = P4_ENTRY_SIZE - 1;
pub const USER_STACK_SIZE: u64 = 0x100_0000;

pub const USER_VGA: u64 = 2*P4_ENTRY_SIZE;

pub const KB_DATA_PORT: u16 = 0x60;
pub const KB_STATUS_PORT: u16 = 0x64;
