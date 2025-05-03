use macroquad::prelude::*;

pub struct Config {
    pub left: KeyCode,
    pub right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_cw: KeyCode,
    pub rotate_ccw: KeyCode,
    pub rotate_180: KeyCode,
    pub hold: KeyCode,
    pub reset: KeyCode,
    pub gravity: f32, // Measured in blocks per second
    pub grace_period: u32, // Milliseconds before gravity places piece that is touching floor
    pub das: u32, // Milliseconds before delayed auto-shift activates
    pub arr: u32, // Milliseconds between each movement repetition during DAS
    pub sdr: u32, // Milliseconds to fall one unit when soft dropping
}

impl Config {
    pub fn default() -> Self {
        Self {
            left: KeyCode::J,
            right: KeyCode::L,
            soft_drop: KeyCode::K,
            hard_drop: KeyCode::Space,
            rotate_cw: KeyCode::D,
            rotate_ccw: KeyCode::A,
            rotate_180: KeyCode::S,
            hold: KeyCode::W,
            reset: KeyCode::R,
            gravity: 2.0,
            grace_period: 750,
            das: 100,
            arr: 0,
            sdr: 0,
        }
    }
}
