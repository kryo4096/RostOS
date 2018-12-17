use alloc::collections::VecDeque;
use spin::{Once, Mutex};

type ScancodeQueue = VecDeque<u8>;

static SCANCODE_QUEUE : Once<Mutex<ScancodeQueue>> = Once::new();

pub fn pop_scancode() -> Option<u8> {
    SCANCODE_QUEUE.call_once(||Mutex::new(ScancodeQueue::new())).lock().pop_back()
}

pub fn push_scancode(scancode: u8) {
    SCANCODE_QUEUE.call_once(||Mutex::new(ScancodeQueue::new())).lock().push_front(scancode)
}