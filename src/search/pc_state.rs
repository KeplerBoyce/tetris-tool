use crate::state::{Board, Game, Piece};
use super::{get_locations, Placement};

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
        // Counting number of minos on board
        let mut mino_count = 0;

        for r in 0..23 {
            for c in 0..10 {
                if self.board.tiles[r as usize][c].piece.is_some() {
                    mino_count += 1;
                    // If we have a mino above PC height
                    if r < 23 - self.height {
                        return true;
                    }
                }
            }
        }
        // If we have a number of minos that won't allow PC
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

        // If there are any horizontally separated open regions that don't have a multiple of 4
        // empty squares, it's unsolvable (skims can't save it)
        let mut num_open = 0;

        for c in 0..10 {
            let mut walled_off = true;
            let mut open_this_col = 0;

            for r in (23 - self.height)..23 {
                if self.board.tiles[r as usize][c].piece.is_some() {
                    continue;
                }
                open_this_col += 1;
                // If this tile is open and left tile is also open, then it isn't walled off
                if c > 0 && self.board.tiles[r as usize][c - 1].piece.is_none() {
                    walled_off = false;
                }
            }
            if walled_off {
                // If we have walled off the left region and it doesn't have a multiple of 4 empty
                // squares, then it's unsolvable
                if num_open % 4 != 0 {
                    return true;
                }
                num_open = 0;
            }
            num_open += open_this_col;
        }
        // Also check again at the very end -- this last region doesn't get checked inside the loop
        if num_open % 4 != 0 {
            return true;
        }

        // If none of these checks fail, we have to keep exploring this node
        return false;
    }

    pub fn successors(&self, queue: &Vec<Piece>) -> Vec<(Self, Placement)> {
        let locations;
        if let Some(piece) = self.piece {
            locations = get_locations(&self.board, piece);
        } else {
            return Vec::new();
        }
        let mut successors: Vec<(Self, Placement)> = Vec::new();
        
        // Add all possible next placements for this piece as successors
        for &successor in locations.iter() {
            let mut new_board = self.board.with_placement(
                successor.piece,
                successor.row as u8,
                successor.col as u8,
                successor.rotation,
            );
            let num_cleared = new_board.clear_lines();
            let successor_state = Self {
                board: new_board,
                queue_used: self.queue_used + 1,
                piece: queue.get(self.queue_used as usize).copied(),
                hold: self.hold,
                height: self.height - num_cleared,
            };
            // If the successor will definitely fail, don't explore
            if successor_state.fails_early(queue) {
                continue;
            }
            successors.push((successor_state, Placement::place(
                successor.piece,
                successor.row as u8,
                successor.col as u8,
                successor.rotation,
            )));
        }
        // Adding state for swapping with hold piece
        if let Some(hold) = self.hold {
            successors.push((Self {
                board: self.board,
                queue_used: self.queue_used,
                piece: Some(hold),
                hold: self.piece,
                height: self.height,
            }, Placement::Hold));
        } else {
            successors.push((Self {
                board: self.board,
                queue_used: self.queue_used + 1,
                piece: queue.get(self.queue_used as usize).copied(),
                hold: self.piece,
                height: self.height,
            }, Placement::Hold));
        }
        successors
    }
}
