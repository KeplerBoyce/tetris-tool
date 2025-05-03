use macroquad::prelude::*;

pub const MARGIN: f32 = 10.0;
pub const GRID_THICKNESS: f32 = 2.0;
pub const BOARD_GAP: f32 = 30.0;
pub const QUEUE_GAP: f32 = 20.0;

#[inline(always)]
pub fn tile_size() -> f32 {
    board_width() / 10.0
}

#[inline(always)]
pub fn board_width() -> f32 {
    ((screen_width() / 2.0 - MARGIN).min(screen_height() / 2.3 - MARGIN) as i32 / 10 * 10) as f32
}

#[inline(always)]
pub fn board_x() -> f32 {
    screen_width() / 2.0 - board_width() / 2.0
}

#[inline(always)]
pub fn board_y() -> f32 {
    (screen_height() - board_height()) / 2.0
}

#[inline(always)]
pub fn board_height() -> f32 {
    board_width() * 2.3
}

#[inline(always)]
pub fn hold_x() -> f32 {
    board_x() - hold_width() - BOARD_GAP
}

#[inline(always)]
pub fn hold_width() -> f32 {
    board_width() / 2.0
}

#[inline(always)]
pub fn queue_x() -> f32 {
    board_x() + board_width() + BOARD_GAP
}

#[inline(always)]
pub fn queue_width() -> f32 {
    board_width() / 2.0
}
