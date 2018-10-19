use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

static TIME: AtomicU64 = AtomicU64::new(0);

pub fn tick() {
    TIME.fetch_add(1,Ordering::SeqCst);
}

pub fn get() -> u64 {
    TIME.load(Ordering::SeqCst)
}

pub fn wait(ticks: u64) {
    let time = get();

    while get() - time <= ticks {}
}