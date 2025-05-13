use macroquad::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use crate::search::Placement;
use crate::state::Board;
use crate::state::Piece::{self, *};
use crate::util::font::text_small;
use crate::util::window::*;
use super::SetupState;

#[derive(Clone, PartialOrd, Ord)]
pub struct PcSetup {
    pub name: String,
    pub placements: Vec<Placement>,
}

impl PcSetup {
    pub fn new(name: &str, placements: Vec<Placement>) -> Self {
        Self {
            name: name.to_string(),
            placements,
        }
    }

    // Returns list of remaining placements and the total number of minos filled.
    fn get_remaining_placements(&self, board: &Board) -> (Vec<Placement>, u8) {
        let mut new_placements: Vec<Placement> = Vec::new();
        let mut minos_filled = 0;

        for &placement in self.placements.iter() {
            if let Placement::Place { piece, row, col, rotation } = placement {
                let mut piece_minos_filled = 0;
                for &(offset_row, offset_col) in piece.offset_map(rotation).iter() {
                    let r = (row as i8 + offset_row) as usize;
                    let c = (col as i8 + offset_col) as usize;
                    if board.tiles[r][c].piece.is_some() {
                        piece_minos_filled += 1;
                    }
                }
                minos_filled += piece_minos_filled;
                // If we successfully filled in the whole piece, remove it from placements
                if piece_minos_filled == 4 {
                    // Bypass the part where we add this to the vec, since we want to remove it
                    continue;
                }
                new_placements.push(placement);
            }
        }
        (new_placements, minos_filled)
    }

    pub fn can_build(&self, board: &Board, queue: VecDeque<Piece>, piece: Option<Piece>, hold: Option<Piece>) -> bool {
        // First, eliminate pieces that are already filled by the board
        let (remaining_placements, minos_filled) = self.get_remaining_placements(board);
        // If we have already built the whole thing, don't want to include this in setup list
        if remaining_placements.len() == 0 {
            return false;
        }
        // If any pieces were partially filled in the setup, can't build
        if (self.placements.len() - remaining_placements.len()) * 4 != minos_filled as usize {
            return false;
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

        // First, check if our pieces even match the pieces used by the setup
        let mut setup_counts: HashMap<Piece, u8> = HashMap::new();
        let mut queue_counts: HashMap<Piece, u8> = HashMap::new();
        // Count number of each piece in the setup
        for &placement in remaining_placements.iter() {
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
            SetupState::new(*board, remaining_placements.clone(), queue.clone(), piece, hold),
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

    fn get_final_board(&self, board: &Board) -> Board {
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
        final_board
    }

    // Returns the height of the board diagram drawn
    pub fn draw(&self, board: &Board, x: f32, y: f32, scale: f32, font: Font) -> f32 {
        // Draw name
        let mut height = text_size_small() + margin();
        draw_text_ex(&self.name, x + margin(), y + height, text_small(font, WHITE));
        height += margin();

        // First, construct final board position
        let final_board = self.get_final_board(board);
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
        // Finally, do the board drawing
        for r in start_row..23 {
            for c in 0..10 {
                if let Some(piece) = final_board.tiles[r][c].piece {
                    // If the mino was already there in the original board, draw in gray
                    if let Some(_) = board.tiles[r][c].piece {
                        draw_rectangle(
                            x + tile_size() * scale * c as f32,
                            y + height + tile_size() * scale * (r - start_row) as f32,
                            tile_size() * scale,
                            tile_size() * scale,
                            GRAY,
                        );
                    } else {
                        draw_rectangle(
                            x + tile_size() * scale * c as f32,
                            y + height + tile_size() * scale * (r - start_row) as f32,
                            tile_size() * scale,
                            tile_size() * scale,
                            piece.color(),
                        );
                    }
                }
            }
        }
        (23 - start_row) as f32 * scale * tile_size() + height
    }
}

impl PartialEq for PcSetup {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PcSetup {}

impl Hash for PcSetup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
