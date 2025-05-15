use std::collections::VecDeque;
use crate::search::{get_locations, Placement, SearchState};
use crate::state::{Board, Piece};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetupState {
    pub board: Board,
    pub placements: Vec<Placement>,
    pub queue: VecDeque<Piece>,
    pub piece: Option<Piece>,
    pub hold: Option<Piece>,
    pub held: bool, // True when we have already swapped for hold piece on this move
}

impl SetupState {
    pub fn new(
        board: Board,
        placements: Vec<Placement>,
        queue: VecDeque<Piece>,
        piece: Option<Piece>,
        hold: Option<Piece>,
        held: bool,
    ) -> Self {
        Self {
            board,
            placements,
            queue,
            piece,
            hold,
            held,
        }
    }

    pub fn successors(&self) -> Vec<Self> {
        let mut list: Vec<Self> = Vec::new();
        let mut successor;
        // Placing next piece options
        for i in 0..self.placements.len() {
            if let Placement::Place{ piece, row, col, rotation } = self.placements[i] {
                if self.piece == Some(piece) {
                    // Check if it is possible to place the piece here
                    let state = SearchState::new(row as i8, col as i8, rotation, piece);
                    let locations = get_locations(&self.board, piece);
                    if !locations.contains(&state) && !locations.contains(&state.symmetrical()) {
                        continue;
                    }
                    // If it is possible, add this successor
                    successor = self.clone();
                    if let Placement::Place { piece, row, col, rotation } = successor.placements.swap_remove(i) {
                        successor.board = successor.board.with_placement(piece, row, col, rotation);
                    }
                    successor.piece = successor.queue.pop_front();
                    successor.held = false;
                    list.push(successor);
                }
            }
        }
        // Hold option -- only allow if hold hasn't been used yet
        if !self.held {
            successor = self.clone();
            if let Some(hold) = self.hold {
                successor.hold = self.piece;
                successor.piece = Some(hold);
            } else {
                successor.hold = self.piece;
                successor.piece = successor.queue.pop_front();
            }
            list.push(successor);
        }
        list
    }
}
