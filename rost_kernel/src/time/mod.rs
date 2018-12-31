use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

use x86_64::instructions::port::{Port, PortReadWrite};

const CHANNEL_0: Port<u8> = Port::new(0x40);

pub fn set_interval(micros: u64) {
    const BASE_NANOS: u64 = 838;

    let mult = micros * 1000 / BASE_NANOS;

    unsafe {
        CHANNEL_0.write((mult & 0xff) as u8);
        CHANNEL_0.write((mult >> 8) as u8);
    }
}

static TIME: AtomicU64 = AtomicU64::new(0);

pub fn tick() {
    TIME.fetch_add(1, Ordering::SeqCst);
}

pub fn get() -> u64 {
    TIME.load(Ordering::SeqCst)
}

pub fn wait(ticks: u64) {
    let time = get();

    while get() - time <= ticks {}
}
