use rand::rngs::StdRng;
use rand::SeedableRng;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref RNG: Mutex<StdRng> = {
        let seed = 100;
        Mutex::new(StdRng::seed_from_u64(seed))
    };
}

pub fn rng<'a>() -> MutexGuard<'a, StdRng> {
    match RNG.lock() {
        Ok(rng) => {
            rng
        },
        Err(poisoned) => {
            eprintln!("Mutex was poisoned!");
            poisoned.into_inner()
        },
    }
}
