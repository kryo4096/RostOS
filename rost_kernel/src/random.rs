extern crate rand;

use spin::{RwLock, Once};
use self::rand::rngs::SmallRng;
use self::rand::prelude::*;

static RNG : Once<RwLock<SmallRng>> = Once::new();

pub fn random_int() -> i64 {
    let ret: i64;
    unsafe {
        asm!("rdrand $0" :"=r"(ret):::);
    }

    //RNG.call_once(||RwLock::new(SmallRng::from_seed([12;16]))).write().next_u64() as _
    ret
}