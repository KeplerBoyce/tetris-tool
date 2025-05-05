use crate::state::{Board, Piece, Rotation};
use super::Movement;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SearchState {
    row: i8,
    col: i8,
    rotation: Rotation,
    piece: Piece,
}

impl SearchState {
    pub fn new(row: i8, col: i8, rotation: Rotation, piece: Piece) -> Self {
        Self {
            row,
            col,
            rotation,
            piece,
        }
    }

    pub fn intersects(&self, board: &Board) -> bool {
        for &(offset_row, offset_col) in self.piece.offset_map(self.rotation).iter() {
            let r = self.row as i8 + offset_row;
            let c = self.col as i8 + offset_col;
            if r < 0 || r > 22 || c < 0 || c > 9 || board.tiles[r as usize][c as usize].piece.is_some() {
                return true;
            }
        }
        false
    }

    pub fn successors(&self, board: &Board) -> Vec<(Self, Movement)> {
        vec![
            (self.left(board), Movement::Left),
            (self.das_left(board), Movement::DasLeft),
            (self.right(board), Movement::Right),
            (self.das_right(board), Movement::DasRight),
            (self.drop(board), Movement::SoftDrop),
            (self.rotate_cw(board), Movement::RotateCw),
            (self.rotate_ccw(board), Movement::RotateCcw),
            (self.rotate_180(board), Movement::Rotate180),
        ]
    }

    pub fn left(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        new_state.col -= 1;
        if new_state.intersects(board) {
            new_state.col += 1;
        }
        new_state
    }

    pub fn das_left(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        loop {
            new_state.col -= 1;
            if new_state.intersects(board) {
                new_state.col += 1;
                break;
            }
        }
        new_state
    }

    pub fn right(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        new_state.col += 1;
        if new_state.intersects(board) {
            new_state.col -= 1;
        }
        new_state
    }

    pub fn das_right(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        loop {
            new_state.col += 1;
            if new_state.intersects(board) {
                new_state.col -= 1;
                break;
            }
        }
        new_state
    }

    pub fn drop(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        loop {
            new_state.row += 1;
            if new_state.intersects(board) {
                new_state.row -= 1;
                break;
            }
        }
        new_state
    }

    pub fn rotate_cw(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        new_state.rotation = match self.rotation {
            Rotation::Normal => Rotation::Cw,
            Rotation::Cw => Rotation::Flip,
            Rotation::Ccw => Rotation::Normal,
            Rotation::Flip => Rotation::Ccw,
        };
        // Handle weird I rotations
        if new_state.piece == Piece::I {
            match new_state.rotation {
                Rotation::Normal => {
                    new_state.row -= 1;
                },
                Rotation::Cw => {
                    new_state.col += 1;
                },
                Rotation::Ccw => {
                    new_state.col -= 1;
                },
                Rotation::Flip => {
                    new_state.row += 1;
                },
            }
        }
        new_state.apply_kicks(board, self.rotation);
        new_state
    }

    pub fn rotate_ccw(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        new_state.rotation = match self.rotation {
            Rotation::Normal => Rotation::Ccw,
            Rotation::Cw => Rotation::Normal,
            Rotation::Ccw => Rotation::Flip,
            Rotation::Flip => Rotation::Cw,
        };
        // Handle weird I rotations
        if new_state.piece == Piece::I {
            match new_state.rotation {
                Rotation::Normal => {
                    new_state.col -= 1;
                },
                Rotation::Cw => {
                    new_state.row -= 1;
                },
                Rotation::Ccw => {
                    new_state.row += 1;
                },
                Rotation::Flip => {
                    new_state.col += 1;
                },
            }
        }
        new_state.apply_kicks(board, self.rotation);
        new_state
    }

    pub fn rotate_180(&self, board: &Board) -> Self {
        let mut new_state = self.clone();
        new_state.rotation = match self.rotation {
            Rotation::Normal => Rotation::Flip,
            Rotation::Cw => Rotation::Ccw,
            Rotation::Ccw => Rotation::Cw,
            Rotation::Flip => Rotation::Normal,
        };
        // Handle weird I rotations
        if new_state.piece == Piece::I {
            match new_state.rotation {
                Rotation::Normal => {
                    new_state.row -= 1;
                    new_state.col -= 1;
                },
                Rotation::Cw => {
                    new_state.row -= 1;
                    new_state.col += 1;
                },
                Rotation::Ccw => {
                    new_state.row += 1;
                    new_state.col -= 1;
                },
                Rotation::Flip => {
                    new_state.row += 1;
                    new_state.col += 1;
                },
            }
        }
        new_state.apply_kicks(board, self.rotation);
        new_state
    }

    fn apply_kicks(&mut self, board: &Board, old_rot: Rotation) {
        let original = self.clone();
        for &(kick_row, kick_col) in self.piece.kick_map(old_rot, self.rotation).iter() {
            self.row = original.row + kick_row;
            self.col = original.col + kick_col;
            if self.intersects(board) {
                continue;
            }
            // If we got here, this kick succeeded -- return
            return;
        }
        // If none of the kicks worked, leave original state
        self.row = original.row;
        self.col = original.col;
    }

    pub fn symmetrical(&self) -> Self {
        match (self.piece, self.rotation) {
            (Piece::I, Rotation::Normal) => Self {
                row: self.row,
                col: self.col + 1,
                rotation: Rotation::Flip,
                piece: Piece::I,
            },
            (Piece::I, Rotation::Cw) => Self {
                row: self.row + 1,
                col: self.col,
                rotation: Rotation::Ccw,
                piece: Piece::I,
            },
            (Piece::I, Rotation::Ccw) => Self {
                row: self.row - 1,
                col: self.col,
                rotation: Rotation::Cw,
                piece: Piece::I,
            },
            (Piece::I, Rotation::Flip) => Self {
                row: self.row,
                col: self.col - 1,
                rotation: Rotation::Normal,
                piece: Piece::I,
            },
            (Piece::S, Rotation::Normal) => Self {
                row: self.row - 1,
                col: self.col,
                rotation: Rotation::Flip,
                piece: Piece::S,
            },
            (Piece::S, Rotation::Cw) => Self {
                row: self.row,
                col: self.col + 1,
                rotation: Rotation::Ccw,
                piece: Piece::S,
            },
            (Piece::S, Rotation::Ccw) => Self {
                row: self.row,
                col: self.col - 1,
                rotation: Rotation::Cw,
                piece: Piece::S,
            },
            (Piece::S, Rotation::Flip) => Self {
                row: self.row + 1,
                col: self.col,
                rotation: Rotation::Normal,
                piece: Piece::S,
            },
            (Piece::Z, Rotation::Normal) => Self {
                row: self.row - 1,
                col: self.col,
                rotation: Rotation::Flip,
                piece: Piece::Z,
            },
            (Piece::Z, Rotation::Cw) => Self {
                row: self.row,
                col: self.col + 1,
                rotation: Rotation::Ccw,
                piece: Piece::Z,
            },
            (Piece::Z, Rotation::Ccw) => Self {
                row: self.row,
                col: self.col - 1,
                rotation: Rotation::Cw,
                piece: Piece::Z,
            },
            (Piece::Z, Rotation::Flip) => Self {
                row: self.row + 1,
                col: self.col,
                rotation: Rotation::Normal,
                piece: Piece::Z,
            },
            _ => *self,
        }
    }
}
