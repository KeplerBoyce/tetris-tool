use macroquad::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::search::Placement;
use crate::state::Board;
use crate::state::Piece::{self, *};
use crate::util::window::tile_size;
use super::SetupState;

#[derive(Clone)]
pub struct PcSetup {
    pub placements: Vec<Placement>,
}

impl PcSetup {
    pub fn new(placements: Vec<Placement>) -> Self {
        Self {
            placements,
        }
    }

    pub fn can_build(&mut self, board: &Board, queue: VecDeque<Piece>, piece: Option<Piece>, hold: Option<Piece>) -> bool {
        // First, eliminate pieces that are already filled by the board
        let mut minos_filled = 0;
        let mut i = 0;

        while i < self.placements.len() {
            let placement = self.placements[i];

            if let Placement::Place { piece, row, col, rotation } = placement {
                let mut piece_minos_filled = 0;
                for &(offset_row, offset_col) in piece.offset_map(rotation).iter() {
                    let r = (row as i8 + offset_row) as usize;
                    let c = (col as i8 + offset_col) as usize;
                    if board.tiles[r][c].piece.is_some() {
                        piece_minos_filled += 1;
                    }
                }
                // If we successfully filled in the whole piece, remove it from placements
                if piece_minos_filled == 4 {
                    minos_filled += 4;
                    self.placements.remove(i);
                    // Bypass the i += 1 since we removed something
                    continue;
                } else if piece_minos_filled > 0 {
                    // This means a piece was only partially filled, so we can't build
                    return false;
                }
            }
            i += 1;
        }
        // Check if we filled in all of the minos on the board -- if not, the board has junk that
        // will prevent the setup from being buildable
        let mut board_mino_count = 0;
        for r in 3..23 {
            for c in 0..10 {
                if board.tiles[r][c].piece.is_some() {
                    board_mino_count += 1;
                }
            }
        }
        if board_mino_count != minos_filled {
            return false;
        }
        // If we have already built the whole thing, don't want to include this in setup list
        if self.placements.len() == 0 {
            return false;
        }

        // First, check if our pieces even match the pieces used by the setup
        let mut setup_counts: HashMap<Piece, u8> = HashMap::new();
        let mut queue_counts: HashMap<Piece, u8> = HashMap::new();
        // Count number of each piece in the setup
        for &placement in self.placements.iter() {
            if let Placement::Place { piece, .. } = placement {
                let count = setup_counts.entry(piece).or_insert(0);
                *count += 1;
            }
        }
        // Count number of each piece in the queue
        for &p in queue.iter() {
            let count = queue_counts.entry(p).or_insert(0);
            *count += 1;
        }
        // Also add the current piece and hold piece
        if let Some(p) = piece {
            let count = queue_counts.entry(p).or_insert(0);
            *count += 1;
        }
        if let Some(p) = hold {
            let count = queue_counts.entry(p).or_insert(0);
            *count += 1;
        }

        // If there is any piece for which the setup needs more than the queue has, can't build
        for &p in &[I, J, L, O, S, T, Z] {
            if setup_counts.get(&p).unwrap_or(&0) > queue_counts.get(&p).unwrap_or(&0) {
                return false;
            }
        }

        // If we have a sufficient number of pieces, then need to check if we can actually place
        // them in the right places in the right order. This is super not ideal to be cloning two
        // vecs for every state in the stack, but the search should be (extremely) short so this is
        // pretty negligible.

        // This stack contains all of the remaining placements that need to be made as well as the
        // current state of the queue, piece, and hold piece.
        let mut stack: Vec<SetupState> = vec![
            SetupState::new(*board, self.placements.clone(), queue.clone(), piece, hold),
        ];
        let mut visited: HashSet<SetupState> = HashSet::new();

        while let Some(state) = stack.pop() {
            // Successfully built
            if state.placements.len() == 0 {
                return true;
            }
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());
            // Iterate over successors and add ones that haven't been visited
            for successor in state.successors().iter() {
                if visited.contains(successor) {
                    continue;
                }
                stack.push(successor.clone());
            }
        }
        // If we weren't able to find a build solution in the DFS, can't build
        false
    }

    // Returns the height of the board diagram drawn
    pub fn draw(&self, board: &Board, x: f32, y: f32, scale: f32) -> f32 {
        // First, construct final board position
        let mut final_board = *board;
        for &placement in self.placements.iter() {
            if let Placement::Place{ piece, row, col, rotation } = placement {
                for &(offset_row, offset_col) in piece.offset_map(rotation).iter() {
                    let r = (row as i8 + offset_row) as usize;
                    let c = (col as i8 + offset_col) as usize;
                    final_board.tiles[r][c].piece = Some(piece);
                }
            }
        }
        // Then, find the highest row in the board from which to start drawing
        let mut start_row = 0;
        'row: for r in 0..23 {
            for c in 0..10 {
                if final_board.tiles[r][c].piece.is_some() {
                    start_row = r;
                    break 'row;
                }
            }
        }
        // Finally, do the drawing
        for r in start_row..23 {
            for c in 0..10 {
                if let Some(piece) = final_board.tiles[r][c].piece {
                    // If the mino was already there in the original board, draw in gray
                    if let Some(_) = board.tiles[r][c].piece {
                        draw_rectangle(
                            x + tile_size() * scale * c as f32,
                            y + tile_size() * scale * (r - start_row) as f32,
                            tile_size() * scale,
                            tile_size() * scale,
                            GRAY,
                        );
                    } else {
                        draw_rectangle(
                            x + tile_size() * scale * c as f32,
                            y + tile_size() * scale * (r - start_row) as f32,
                            tile_size() * scale,
                            tile_size() * scale,
                            piece.color(),
                        );
                    }
                }
            }
        }
        (23 - start_row) as f32 * scale * tile_size()
    }
}
