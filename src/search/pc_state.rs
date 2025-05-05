use crate::state::{Board, Game, Piece};
use super::get_locations;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PcState {
    pub board: Board,
    pub queue_used: u8, // How many pieces from queue have been used
    pub piece: Option<Piece>,
    pub hold: Option<Piece>,
    pub height: u8, // Height of this PC (number of lines)
}

impl PcState {
    pub fn from(game: &Game, height: u8) -> Self {
        Self {
            board: game.board,
            queue_used: 0,
            piece: game.piece,
            hold: game.hold,
            height,
        }
    }

    pub fn is_solved(&self) -> bool {
        for c in 0..10 {
            for r in 0..(23 - self.height) {
                if self.board.tiles[r as usize][c].piece.is_some() {
                    return false;
                }
            }
            for r in (23 - self.height)..23 {
                if self.board.tiles[r as usize][c].piece.is_none() {
                    return false;
                }
            }
        }
        true
    }

    // Returns true if this state will obviously fail to PC -- allows use of backtracking search to
    // cut down search time
    pub fn fails_early(&self, queue: &Vec<Piece>) -> bool {
        // If we have a mino above the PC height
        for r in 0..(23 - self.height) {
            for c in 0..10 {
                if self.board.tiles[r as usize][c].piece.is_some() {
                    return true;
                }
            }
        }
        // If we have a number of minos that won't allow PC
        let mut mino_count = 0;
        for r in (23 - self.height)..23 {
            for c in 0..10 {
                if self.board.tiles[r as usize][c].piece.is_some() {
                    mino_count += 1;
                }
            }
        }
        if (10 * self.height - mino_count) % 4 != 0 {
            return true;
        }
        // If number of pieces placed + pieces in queue isn't enough minos
        let minos_to_place = (queue.len() + 1 + if self.hold.is_some() {
            1
        } else {
            0
        }) * 4;
        if minos_to_place as u8 + mino_count < 10 * self.height {
            return true;
        }
        return false;
    }

    pub fn successors(&self, queue: &Vec<Piece>) -> Vec<Self> {
        let locations;
        if let Some(piece) = self.piece {
            locations = get_locations(&self.board, piece);
        } else {
            return Vec::new();
        }
        let mut successors: Vec<Self> = Vec::new();
        
        // Add all possible next placements for this piece as successors
        for &successor in locations.iter() {
            let successor_state = Self {
                board: self.board.with_placement(
                    successor.piece,
                    successor.row as u8,
                    successor.col as u8,
                    successor.rotation,
                ),
                queue_used: self.queue_used + 1,
                piece: queue.get(self.queue_used as usize).copied(),
                hold: self.hold,
                height: self.height,
            };
            // If the successor will definitely fail, don't explore
            if successor_state.fails_early(queue) {
                continue;
            }
            successors.push(successor_state);
        }
        // Adding state for swapping with hold piece
        if let Some(hold) = self.hold {
            successors.push(Self {
                board: self.board,
                queue_used: self.queue_used,
                piece: Some(hold),
                hold: self.piece,
                height: self.height,
            });
        } else {
            successors.push(Self {
                board: self.board,
                queue_used: self.queue_used + 1,
                piece: queue.get(self.queue_used as usize).copied(),
                hold: self.piece,
                height: self.height,
            });
        }
        successors
    }
}
