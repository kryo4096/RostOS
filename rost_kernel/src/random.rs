extern crate rand;

use self::rand::prelude::*;
use self::rand::rngs::SmallRng;
use spin::{Once, RwLock};

static RNG: Once<RwLock<SmallRng>> = Once::new();

pub fn random_int() -> i64 {
    let ret;
    unsafe {
        asm!("rdrand $0" :"=r"(ret):::);
    }

    //RNG.call_once(||RwLock::new(SmallRng::from_seed([12;16]))).write().next_u64() as _
    ret
}

pub fn random_uint() -> u64 {
    let ret;
    unsafe {
        asm!("rdrand $0" :"=r"(ret):::);
    }

    //RNG.call_once(||RwLock::new(SmallRng::from_seed([12;16]))).write().next_u64() as _
    ret
}
