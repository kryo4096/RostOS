use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use consts::USER_SIGNAL_STACK_TOP;
use process;
use process::{Process, State, WaitReason};

use spin::{Mutex, MutexGuard, Once};

static SIGNAL_BUS: Once<Mutex<SignalBus>> = Once::new();

pub fn signal_bus() -> MutexGuard<'static, SignalBus> {
    SIGNAL_BUS.call_once(|| Mutex::new(SignalBus::new())).lock()
}

extern "C" {
    fn _call_signal(
        arg0: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        handler_addr: u64,
        stack_pointer: u64,
    );
}

use core::sync::atomic::{AtomicBool, Ordering};

struct SignalSubscriber {
    channel: u64,
    pid: u64,
    handler_addr: u64,
    ready: AtomicBool,
}

impl SignalSubscriber {
    pub unsafe fn call(&self, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> bool {
        if let Some(process) = Process::get(self.pid) {
            if let State::Waiting(WaitReason::ProcessExit(_)) = process.read().state {
                return true;
            }
        }

        if let Some(old_pid) = process::load_space(self.pid) {
            _call_signal(
                arg0,
                arg1,
                arg2,
                arg3,
                self.handler_addr,
                USER_SIGNAL_STACK_TOP,
            );
            self.ready.store(true, Ordering::SeqCst);

            process::load_space(old_pid).unwrap();
            true
        } else {
            false
        }
    }
}

pub struct SignalBus {
    channels: BTreeMap<u64, Vec<SignalSubscriber>>,
    max_id: u64,
}

impl SignalBus {
    fn new() -> Self {
        let mut channels = BTreeMap::new();

        Self {
            channels,
            max_id: 0,
        }
    }

    pub fn add_channel(&mut self, channel: u64) {
        assert!(self.channels.insert(channel, Vec::new()).is_none());
        if channel > self.max_id {
            self.max_id = channel;
        }
    }

    pub fn alloc_channel(&mut self) -> u64 {
        let channel = self.max_id + 1;
        self.add_channel(channel);
        channel
    }

    pub fn has_channel(&self, channel: u64) -> bool {
        self.channels.contains_key(&channel)
    }

    pub unsafe fn call(
        &mut self,
        channel: u64,
        arg0: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
    ) -> Option<()> {
        self.channels
            .get_mut(&channel)?
            .retain(|subscriber| subscriber.call(arg0, arg1, arg2, arg3));
        Some(())
    }

    pub fn subscribe(&mut self, channel: u64, pid: u64, handler_addr: u64) -> Option<()> {
        self.channels.get_mut(&channel)?.push(SignalSubscriber {
            channel,
            pid,
            handler_addr,
            ready: AtomicBool::new(true),
        });
        Some(())
    }
}
