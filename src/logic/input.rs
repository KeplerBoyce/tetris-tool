use std::time::Instant;
use macroquad::prelude::*;
use crate::state::{Game, Piece, Rotation};
use super::{Config, Stats};

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
                
                if col < 0 || col > 9 || row < 0 || row > 22 ||
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

pub fn handle_input(config: &Config, stats: &mut Stats, game: &mut Game) {
    if is_key_pressed(config.reset) {
        *game = Game::new();
        *stats = Stats::new();
    }

    if is_key_pressed(config.undo) {
        if let Some((board, piece, old_stats)) = game.undo_stack.pop() {
            // Add current piece back into start of queue
            if let Some(curr_piece) = game.piece {
                game.queue.push_front(curr_piece);
            }
            // Restore previous board state and piece
            game.board = board;
            game.piece = piece;
            game.piece_row = 1;
            game.piece_col = 4;
            game.rotation = Rotation::Normal;
            game.prev_stats = old_stats;
            *stats = old_stats;
        }
    }

    let now = Instant::now();

    if is_key_pressed(config.left) {
        stats.inputs += 1;
        game.left_time = now;
        game.left_priority = true;
        game.piece_col -= 1;
        if game.check_wall_intersect() {
            game.piece_col += 1;
        }
    }
    // Handle left movement repetition
    if is_key_down(config.left) {
        if now.duration_since(game.left_time).as_millis() as u32 >= config.das {
            game.left_das_activated = true;
        }
        // If left is more recently held than right and has been held long enough
        if game.left_priority && game.left_das_activated &&
                now.duration_since(game.left_time).as_millis() as u32 >= config.arr {
            game.left_time = now;
            // If ARR is 0 then repeat all the way, otherwise just move once
            if config.arr == 0 {
                loop {
                    game.piece_col -= 1;
                    if game.check_wall_intersect() {
                        game.piece_col += 1;
                        break;
                    }
                }
            } else {
                game.piece_col -= 1;
                if game.check_wall_intersect() {
                    game.piece_col += 1;
                }
            }
        }
    }
    // Reset DAS when key is released
    if is_key_released(config.left) {
        game.left_das_activated = false;
        game.left_priority = false;
    }

    if is_key_pressed(config.right) {
        stats.inputs += 1;
        game.right_time = now;
        game.left_priority = false;
        game.piece_col += 1;
        if game.check_wall_intersect() {
            game.piece_col -= 1;
        }
    }
    // Handle right movement repetition
    if is_key_down(config.right) {
        if now.duration_since(game.right_time).as_millis() as u32 >= config.das {
            game.right_das_activated = true;
        }
        // If right is more recently held than left and has been held long enough
        if !game.left_priority && game.right_das_activated &&
                now.duration_since(game.right_time).as_millis() as u32 >= config.arr {
            game.right_time = now;
            // If ARR is 0 then repeat all the way, otherwise just move once
            if config.arr == 0 {
                loop {
                    game.piece_col += 1;
                    if game.check_wall_intersect() {
                        game.piece_col -= 1;
                        break;
                    }
                }
            } else {
                game.piece_col += 1;
                if game.check_wall_intersect() {
                    game.piece_col -= 1;
                }
            }
        }
    }
    // Reset DAS when key is released
    if is_key_released(config.right) {
        game.right_das_activated = false;
        game.left_priority = true;
    }

    if is_key_pressed(config.soft_drop) {
        stats.inputs += 1;
        game.soft_drop_time = now;
    }
    // Handle soft drop repetition
    if is_key_down(config.soft_drop) {
        if now.duration_since(game.soft_drop_time).as_millis() as u32 >= config.sdr {
            game.soft_drop_time = now;
            // If SDR is 0 then repeat all the way, otherwise just move once
            if config.sdr == 0 {
                loop {
                    game.piece_row += 1;
                    if game.check_landing() {
                        game.piece_row -= 1;
                        break;
                    } else {
                        game.piece_row += 1;
                        // This ensures that the grace period is only applied the first time the piece
                        // touches the ground -- player can't tap soft drop repeatedly to refresh grace
                        // period over and over
                        if game.check_landing() {
                            game.refresh_last_time();
                            game.piece_row -= 1;
                            break;
                        }
                        game.piece_row -= 1;
                    }
                }
            } else {
                game.piece_row += 1;
                if game.check_landing() {
                    game.piece_row -= 1;
                } else {
                    game.piece_row += 1;
                    // Same as above
                    if game.check_landing() {
                        game.refresh_last_time();
                    }
                    game.piece_row -= 1;
                }
            }
        }
    }

    if is_key_pressed(config.hard_drop) {
        stats.inputs += 1;
        loop {
            game.piece_row += 1;
            if game.check_landing() {
                game.piece_row -= 1;
                game.place_piece(stats);
                break;
            }
        }
    }

    if is_key_pressed(config.rotate_cw) {
        stats.inputs += 1;
        let old_rot = game.rotation;
        apply_cw(game);
        if !attempt_kicks(game, old_rot) {
            apply_ccw(game);
        } else {
            game.piece_row += 1;
            if game.check_landing() {
                game.refresh_last_time();
            }
            game.piece_row -= 1;
        }
    }
    if is_key_pressed(config.rotate_ccw) {
        stats.inputs += 1;
        let old_rot = game.rotation;
        apply_ccw(game);
        if !attempt_kicks(game, old_rot) {
            apply_cw(game);
        } else {
            game.piece_row += 1;
            if game.check_landing() {
                game.refresh_last_time();
            }
            game.piece_row -= 1;
        }
    }
    if is_key_pressed(config.rotate_180) {
        stats.inputs += 1;
        let old_rot = game.rotation;
        apply_180(game);
        if !attempt_kicks(game, old_rot) {
            apply_180(game);
        } else {
            game.piece_row += 1;
            if game.check_landing() {
                game.refresh_last_time();
            }
            game.piece_row -= 1;
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
