use std::{collections::HashSet, iter};
use crate::state::{Board, Game, Piece};
use super::{get_locations, SearchState};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PcState {
    pub board: Board,
    pub queue: Vec<Piece>,
    pub hold: Option<Piece>,
    pub height: u8, // Height of this PC (number of lines)
}

impl PcState {
    pub fn from(game: &Game) -> Self {
        let mut queue = game.queue.clone();
        if let Some(piece) = game.piece {
            queue.push_front(piece);
        }
        Self {
            board: game.board,
            queue: Vec::from(queue),
            hold: game.hold,
            height: 4,
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

    pub fn successors(&self) -> Vec<Self> {
        if let Some(&piece) = self.queue.first() {
            let mut successors: Vec<Self> = Vec::new();
            let locations = get_locations(&self.board, piece);
            let mut visited: HashSet<SearchState> = HashSet::new();
            
            // Add all possible next placements for this piece as successors
            for &successor in locations.keys() {
                // Avoid duplicate successors -- different states with same final placement
                if visited.contains(&successor.symmetrical()) {
                    continue;
                }
                visited.insert(successor);
                successors.push(Self {
                    board: self.board.with_placement(
                        successor.piece,
                        successor.row as u8,
                        successor.col as u8,
                        successor.rotation,
                    ),
                    queue: self.queue[1..].to_vec(),
                    hold: self.hold,
                    height: self.height,
                });
            }
            // Adding state for swapping with hold piece
            if let Some(hold) = self.hold {
                successors.push(Self {
                    board: self.board,
                    queue: iter::once(hold).chain(self.queue[1..].iter().cloned()).collect(),
                    hold: Some(piece),
                    height: self.height,
                });
            } else {
                successors.push(Self {
                    board: self.board,
                    queue: self.queue[1..].to_vec(),
                    hold: Some(piece),
                    height: self.height,
                });
            }
            successors
        } else {
            Vec::new()
        }
    }
}
