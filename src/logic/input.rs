use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;
use crossbeam_channel::Sender;
use macroquad::prelude::*;
use crate::state::{Game, Piece, Rotation};
use crate::search::{Movement, Pc};
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

pub fn handle_input(
    config: &Config,
    stats: &mut Stats,
    game: &mut Game,
    waiting: bool,
    cancel_flag: &mut Option<Arc<AtomicBool>>,
    tx: &Sender<Vec<Pc>>,
) {
    // If we are waiting for a keybind input, don't move in the game
    if waiting {
        return;
    }

    if is_key_pressed(config.reset) {
        *game = Game::new();
        *stats = Stats::new();
    }

    if is_key_pressed(config.undo) {
        if let Some((board, piece, hold, held, old_stats, pc_piece_num)) = game.undo_stack.pop() {
            // Add current piece back into start of queue
            if !game.held || hold.is_none() {
                if let Some(curr_piece) = game.piece {
                    game.queue.push_front(curr_piece);
                }
            }
            // Restore previous board state and piece
            game.board = board;
            game.piece = piece;
            game.hold = hold;
            game.held = held;
            game.piece_row = 1;
            game.piece_col = 4;
            game.rotation = Rotation::Normal;
            game.prev_stats = old_stats;
            game.pc_piece_num = pc_piece_num;
            *stats = old_stats;
            // Reset finesse path
            game.my_path = Vec::new();
            // Refresh PC solutions
            game.pcs = Vec::new();
            game.refresh_pcs(cancel_flag, tx);
        }
    }

    let now = Instant::now();

    if is_key_pressed(config.left) {
        stats.inputs += 1;
        game.my_path.push(Movement::Left);
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
            if let Some(last) = game.my_path.last_mut() {
                *last = Movement::DasLeft;
            }
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
        game.my_path.push(Movement::Right);
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
            if let Some(last) = game.my_path.last_mut() {
                *last = Movement::DasRight;
            }
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
        game.my_path.push(Movement::SoftDrop);
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
        game.my_path.push(Movement::HardDrop);
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
        game.my_path.push(Movement::RotateCw);
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
        game.my_path.push(Movement::RotateCcw);
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
        game.my_path.push(Movement::Rotate180);
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
        if !game.held {
            game.undo_stack.push((game.board, game.piece, game.hold, game.held, *stats, game.pc_piece_num));
            let piece = game.piece;
            game.piece = game.hold;
            game.hold = piece;
            game.piece_row = 1;
            game.piece_col = 4;
            game.rotation = Rotation::Normal;
            // Clear path -- resets when you hold to avoid extra faults
            game.my_path = Vec::new();
            game.held = true;
            // Refresh PC solutions because this might have made some impossible
            game.refresh_pcs(cancel_flag, tx);
        }
    }
}
