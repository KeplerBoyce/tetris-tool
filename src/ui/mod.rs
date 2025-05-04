use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;

mod helpers;

pub use helpers::*;

lazy_static! {
    static ref GAME_X: Mutex<f32> = Mutex::new(0.0);
}

pub fn lock_game_x<'a>() -> MutexGuard<'a, f32> {
    match GAME_X.lock() {
        Ok(game_x) => {
            game_x
        },
        Err(poisoned) => {
            eprintln!("Mutex was poisoned!");
            poisoned.into_inner()
        },
    }
}
