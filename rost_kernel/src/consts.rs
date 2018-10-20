pub const P4_TABLE_ADDR: u64 = 0xf_fff_fff_fff_fff_000;
pub const PAGE_SIZE: u64 = 0x1000; // frame and page size is 4 KiB
pub const FRAME_STACK_SIZE: usize = 0x100; // amount of freed frames stored before they are discarded

pub const KERNEL_HEAP_START: u64 = 0x100_0000_0000;
pub const KERNEL_HEAP_SIZE: u64 = 0x100_0000; // 16 MiB

pub const VGA_BUFFER_VADDR: u64 = 0x180_0000_0000;

pub const USER_TEXT_START: u64 = 0x0;           // userspace
pub const USER_STACK_TOP: u64 = 0x7F_FFFF_FFFF; // |   text   |  heap | ... | stack | 