use macroquad::prelude::*;
use crate::ui::lock_game_x;

const SCREEN_MARGIN: f32 = 10.0;

#[inline(always)]
pub fn margin() -> f32 {
    tile_size() / 3.0
}

#[inline(always)]
pub fn grid_thickness() -> f32 {
    ((tile_size() / 15.0) as u32 as f32).max(1.0)
}

#[inline(always)]
pub fn board_gap() -> f32 {
    tile_size() / 2.0
}

#[inline(always)]
pub fn queue_gap() -> f32 {
    tile_size() / 3.0
}

#[inline(always)]
pub fn text_size_small() -> f32 {
    ((0.75 * tile_size()) as u32 / 10 * 10) as f32
}

#[inline(always)]
pub fn text_size_normal() -> f32 {
    (tile_size() as u32 / 10 * 10) as f32
}

#[inline(always)]
pub fn text_size_large() -> f32 {
    ((1.5 * tile_size()) as u32 / 10 * 10) as f32
}

#[inline(always)]
pub fn game_x() -> f32 {
    *lock_game_x()
}

#[inline(always)]
pub fn width() -> f32 {
    screen_width() - game_x()
}

#[inline(always)]
pub fn height() -> f32 {
    screen_height()
}

#[inline(always)]
pub fn tile_size() -> f32 {
    board_width() / 10.0
}

#[inline(always)]
pub fn board_width() -> f32 {
    ((width() / 2.0 - SCREEN_MARGIN).min(height() / 2.3 - SCREEN_MARGIN) as i32 / 10 * 10) as f32
}

#[inline(always)]
pub fn board_x() -> f32 {
    game_x() + width() / 2.0 - board_width() / 2.0
}

#[inline(always)]
pub fn board_y() -> f32 {
    (height() - board_height()) / 2.0
}

#[inline(always)]
pub fn board_height() -> f32 {
    board_width() * 2.3
}

#[inline(always)]
pub fn hold_x() -> f32 {
    board_x() - hold_width() - board_gap()
}

#[inline(always)]
pub fn hold_y() -> f32 {
    board_y() + 3.0 * board_height() / 23.0
}

#[inline(always)]
pub fn hold_width() -> f32 {
    board_width() / 2.0 - board_gap()
}

#[inline(always)]
pub fn hold_height() -> f32 {
    (board_height() - (hold_y() - board_y())) / 4.0
}

#[inline(always)]
pub fn finesse_x() -> f32 {
    hold_x()
}

#[inline(always)]
pub fn finesse_y() -> f32 {
    hold_y() + hold_height()
}

#[inline(always)]
pub fn finesse_width() -> f32 {
    hold_width()
}

#[inline(always)]
pub fn finesse_height() -> f32 {
    (board_height() - (hold_y() - board_y())) / 3.0
}

#[inline(always)]
pub fn queue_x() -> f32 {
    board_x() + board_width() + board_gap()
}

#[inline(always)]
pub fn queue_y() -> f32 {
    board_y() + 3.0 * board_height() / 23.0
}

#[inline(always)]
pub fn queue_width() -> f32 {
    board_width() / 2.0 - board_gap()
}

#[inline(always)]
pub fn queue_height() -> f32 {
    board_height() - (queue_y() - board_y())
}

#[inline(always)]
pub fn piece_num_x() -> f32 {
    queue_x()
}

#[inline(always)]
pub fn piece_num_y() -> f32 {
    board_y()
}

#[inline(always)]
pub fn piece_num_width() -> f32 {
    queue_width()
}

#[inline(always)]
pub fn piece_num_height() -> f32 {
    queue_y() - board_y() - board_gap()
}
