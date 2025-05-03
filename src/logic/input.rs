use macroquad::prelude::*;
use crate::state::{Game, Piece, Rotation};
use super::Config;

fn apply_cw(game: &mut Game) {
    game.rotation = match game.rotation {
        crate::state::Rotation::Normal => Rotation::Cw,
        crate::state::Rotation::Cw => Rotation::Flip,
        crate::state::Rotation::Ccw => Rotation::Normal,
        crate::state::Rotation::Flip => Rotation::Ccw,
    };
    // Handle weird I rotations
    if let Some(piece) = game.piece {
        if piece == Piece::I {
            match game.rotation {
                Rotation::Normal => {
                    game.piece_row -= 1;
                },
                Rotation::Cw => {
                    game.piece_col += 1;
                },
                Rotation::Ccw => {
                    game.piece_col -= 1;
                },
                Rotation::Flip => {
                    game.piece_row += 1;
                },
            }
        }
    }
}

fn apply_ccw(game: &mut Game) {
    game.rotation = match game.rotation {
        crate::state::Rotation::Normal => Rotation::Ccw,
        crate::state::Rotation::Cw => Rotation::Normal,
        crate::state::Rotation::Ccw => Rotation::Flip,
        crate::state::Rotation::Flip => Rotation::Cw,
    };
    // Handle weird I rotations
    if let Some(piece) = game.piece {
        if piece == Piece::I {
            match game.rotation {
                Rotation::Normal => {
                    game.piece_col -= 1;
                },
                Rotation::Cw => {
                    game.piece_row -= 1;
                },
                Rotation::Ccw => {
                    game.piece_row += 1;
                },
                Rotation::Flip => {
                    game.piece_col += 1;
                },
            }
        }
    }
}

fn apply_180(game: &mut Game) {
    game.rotation = match game.rotation {
        crate::state::Rotation::Normal => Rotation::Flip,
        crate::state::Rotation::Cw => Rotation::Ccw,
        crate::state::Rotation::Ccw => Rotation::Cw,
        crate::state::Rotation::Flip => Rotation::Normal,
    };
    // Handle weird I rotations
    if let Some(piece) = game.piece {
        if piece == Piece::I {
            match game.rotation {
                Rotation::Normal => {
                    game.piece_row -= 1;
                    game.piece_col -= 1;
                },
                Rotation::Cw => {
                    game.piece_row -= 1;
                    game.piece_col += 1;
                },
                Rotation::Ccw => {
                    game.piece_row += 1;
                    game.piece_col -= 1;
                },
                Rotation::Flip => {
                    game.piece_row += 1;
                    game.piece_col += 1;
                },
            }
        }
    }
}

fn attempt_kicks(game: &mut Game, old_rot: Rotation) -> bool {
    if let Some(piece) = game.piece {
        // Check all kick possibilities until one succeeds
        'kick: for &(kick_row, kick_col) in piece.kick_map(old_rot, game.rotation).iter() {
            for &(offset_row, offset_col) in piece.offset_map(game.rotation).iter() {
                let row = game.piece_row + kick_row + offset_row;
                let col = game.piece_col + kick_col + offset_col;
                
                if col < 0 || col > 9 || row > 22 ||
                        game.board.tiles[row as usize][col as usize].piece.is_some() {
                    continue 'kick;
                }
            }
            // If we got here, this kick succeeded -- apply kick offset
            game.piece_row += kick_row;
            game.piece_col += kick_col;
            return true;
        }
    }
    // If none of the kicks worked, return false so we know to reverse rotation
    return false;
}

pub fn handle_input(config: &Config, game: &mut Game) {
    if is_key_pressed(config.left) {
        game.piece_col -= 1;
        if game.check_wall_intersect() {
            game.piece_col += 1;
        }
    }
    if is_key_pressed(config.right) {
        game.piece_col += 1;
        if game.check_wall_intersect() {
            game.piece_col -= 1;
        }
    }
    if is_key_pressed(config.soft_drop) {
        let mut moved = false;
        loop {
            game.piece_row += 1;
            if game.check_landing() {
                game.piece_row -= 1;
                if moved {
                    game.refresh_last_time();
                }
                break;
            }
            moved = true;
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
        let old_rot = game.rotation;
        apply_cw(game);
        if !attempt_kicks(game, old_rot) {
            apply_ccw(game);
        }
    }
    if is_key_pressed(config.rotate_ccw) {
        let old_rot = game.rotation;
        apply_ccw(game);
        if !attempt_kicks(game, old_rot) {
            apply_cw(game);
        }
    }
    if is_key_pressed(config.rotate_180) {
        let old_rot = game.rotation;
        apply_180(game);
        if !attempt_kicks(game, old_rot) {
            apply_180(game);
        }
    }
    if is_key_pressed(config.hold) {
        let piece = game.piece;
        game.piece = game.hold;
        game.hold = piece;
        game.piece_row = 1;
        game.piece_col = 4;
        game.rotation = Rotation::Normal;
    }
}
