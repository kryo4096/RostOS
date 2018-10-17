pub const P4_TABLE_ADDR: u64 = 0xf_fff_fff_fff_fff_000;
pub const PAGE_SIZE: u64 = 0x1000; // frame and page size is 4 KiB
pub const FRAME_STACK_SIZE: usize = 0x100; // amount of freed frames stored before they are discarded