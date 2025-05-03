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
        }
    }
}
