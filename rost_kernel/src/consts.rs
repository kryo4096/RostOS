pub const PAGE_SIZE: u64 = 0x1000; // frame and page size is 4 KiB
pub const FRAME_STACK_SIZE: usize = 0x1000; // amount of freed frames stored before they are discarded

pub const P4_TABLE_ADDR: u64 = 0xffff_ffff_ffff_f000;

// MEMORY MAP

pub const MAX_ADDR: u64 = 0xffff_ffff_ffff_ffff; // the highest address in the address space
pub const P4_ENTRY_SIZE: u64 = 0x80_0000_0000; // the amount of memory one level 4 entry spans

pub const KERNEL_START: u64 = MAX_ADDR - P4_ENTRY_SIZE * 256 + 1; // the start of kernel space

pub const KERNEL_HEAP_START: u64 = KERNEL_START + P4_ENTRY_SIZE; // the start of the kernel heap
pub const KERNEL_HEAP_SIZE: u64 = 0x100_0000; // 16 MiB

pub const VGA_BUFFER_VADDR: u64 = KERNEL_START + 2 * P4_ENTRY_SIZE; // the virtual address of the VGA buffer
pub const VGA_BUFFER_PADDR: u64 = 0xb8000; // the physical address of the VGA buffer

pub const RAMDISK_START: u64 = KERNEL_START + 3 * P4_ENTRY_SIZE; // the start of the ramdisk
pub static RAMDISK_SIZE: u64 = ::DISK_IMAGE.len() as u64; // the size of the ramdisk

pub const PT_START: u64 = KERNEL_START + 4 * P4_ENTRY_SIZE; // the start of the processes' page tables

pub const KERNEL_SYSCALL_STACK_START: u64 = KERNEL_START + 128 * P4_ENTRY_SIZE; // 0xffff_82ff_ffff_fffe, start of the user process syscall stacks
pub const KERNEL_SYSCALL_STACK_SIZE: u64 = 0x10_0000; // size of a user process syscall stack

pub const USER_KERNEL_STACK_PTR: u64 = 203 * P4_ENTRY_SIZE; // 0x658000000000, virtual address of the pointer pointing to a process' syscall stack which is different for each process

pub const USER_SIGNAL_STACK_TOP: u64 = 250 * P4_ENTRY_SIZE + 0x10000; // the top of a process' signal stack
pub const USER_SIGNAL_STACK_SIZE: u64 = 0x20000; // the size of a process' signal stack

pub const USER_STACK_TOP: u64 = P4_ENTRY_SIZE + USER_STACK_SIZE; // the top of a process'stack
pub const USER_STACK_SIZE: u64 = 0x100_0000; // the size of a process' stack

// the ps/2 keyboard io ports

pub const KB_DATA_PORT: u16 = 0x60;
pub const KB_STATUS_PORT: u16 = 0x64;
