use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, Size4KiB, PhysFrame};
use super::map::*;

const FRAME_STACK_SIZE: usize = 0x100;

struct FrameStack {
    frames: [usize; FRAME_STACK_SIZE],
    top: usize,
}

impl FrameStack {
    fn empty() -> Self {
        Self {
            frames: [0;FRAME_STACK_SIZE],
            top: 0,
        }
    }

    fn push(&mut self, frame: usize) {
        if self.top < FRAME_STACK_SIZE - 1 {
            self.frames[self.top] = frame;
            self.top += 1;
        }
    } 

    fn pop(&mut self, ) -> Option<usize> {
        if self.top > 0 {
            self.top -= 1;
            Some(self.frames[self.top])
        } else {
            None
        }
    }
}



pub struct FrameStackAllocator {
    memory_map: &'static mut MemoryMap,
    free_stack: FrameStack,
    current_region: Option<usize>, // Region Index Cache
}

impl FrameStackAllocator {
    fn first_free_region(&self) -> Option<usize> {
        self.memory_map.iter().position(MemoryRegion::is_free)
    }

    fn current_region(&mut self) -> usize {
        if let Some(i) = self.current_region{
            i
        } else {                                                                                                                                                                                                                                                                                                                                                                                                                                
            self.current_region = self.first_free_region();
            self.current_region.expect("No more frames available!")
        }
    }
}

