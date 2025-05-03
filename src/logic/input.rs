use macroquad::prelude::*;
use crate::state::{Game, Rotation};
use super::Config;

pub fn handle_input(config: &Config, game: &mut Game) {
    if is_key_pressed(config.left) {
        game.piece_col -= 1;
    }
    if is_key_pressed(config.right) {
        game.piece_col += 1;
    }
    if is_key_pressed(config.soft_drop) {
        loop {
            game.piece_row += 1;
            if game.check_landing() {
                game.piece_row -= 1;
                break;
            }
        }
    }
    if is_key_pressed(config.hard_drop) {
        loop {
            game.piece_row += 1;
            if game.check_landing() {
                game.piece_row -= 1;
                game.place_piece();
                break;
            }
        }
    }
    if is_key_pressed(config.rotate_cw) {
        game.rotation = match game.rotation {
            crate::state::Rotation::Normal => Rotation::Cw,
            crate::state::Rotation::Cw => Rotation::Flip,
            crate::state::Rotation::Ccw => Rotation::Normal,
            crate::state::Rotation::Flip => Rotation::Ccw,
        }
    }
    if is_key_pressed(config.rotate_ccw) {
        game.rotation = match game.rotation {
            crate::state::Rotation::Normal => Rotation::Ccw,
            crate::state::Rotation::Cw => Rotation::Normal,
            crate::state::Rotation::Ccw => Rotation::Flip,
            crate::state::Rotation::Flip => Rotation::Cw,
        }
    }
    if is_key_pressed(config.rotate_180) {
        game.rotation = match game.rotation {
            crate::state::Rotation::Normal => Rotation::Flip,
            crate::state::Rotation::Cw => Rotation::Ccw,
            crate::state::Rotation::Ccw => Rotation::Cw,
            crate::state::Rotation::Flip => Rotation::Normal,
        }
    }
    if is_key_pressed(config.hold) {

    }
}
