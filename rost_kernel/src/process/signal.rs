use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use process;
use consts::USER_SIGNAL_STACK_TOP;

use spin::{Once, Mutex, MutexGuard};

static SIGNAL_BUS: Once<Mutex<SignalBus>> = Once::new(); 

pub fn signal_bus() -> MutexGuard<'static, SignalBus> {
    SIGNAL_BUS.call_once(||Mutex::new(SignalBus::new())).lock()
}


extern "C" {
    fn _call_signal(arg0: u64, arg1: u64, arg2: u64, arg3: u64, handler_addr: u64, stack_pointer: u64);
} 

use core::sync::atomic::{AtomicBool, Ordering};

struct SignalSubscriber {
    pid: u64,
    handler_addr: u64,
    ready: AtomicBool,
}

impl SignalSubscriber {
    pub unsafe fn call(&self, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> bool {
        if let Some(old_pid) = process::load_space(self.pid) {
            if self.ready.compare_and_swap(true,false, Ordering::SeqCst) {
                _call_signal(arg0, arg1, arg2, arg3, self.handler_addr, USER_SIGNAL_STACK_TOP);
                self.ready.store(true, Ordering::SeqCst);
            }
            process::load_space(old_pid);
            true
        } else {
            false
        }
    }
}

pub struct SignalBus {
    channels: BTreeMap<u64, Vec<SignalSubscriber>>,
}

impl SignalBus {
    fn new() -> Self {
        let mut channels = BTreeMap::new();

        Self {
            channels,
        }
    }

    pub fn add_channel(&mut self, channel: u64) {
        self.channels.insert(channel, Vec::new());
    }

    pub fn has_channel(&self, channel: u64) -> bool {
        self.channels.contains_key(&channel)
    }

    pub unsafe fn call(&mut self, channel: u64, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> Option<()> {
        self.channels.get_mut(&channel)?.retain(|subscriber| subscriber.call(arg0,arg1,arg2,arg3));
        Some(())
    }

    pub fn subscribe(&mut self, channel: u64, pid: u64, handler_addr: u64) -> Option<()> {
        self.channels.get_mut(&channel)?.push(SignalSubscriber {
            pid, handler_addr, ready: AtomicBool::new(true),
        });
        Some(())
    }
}