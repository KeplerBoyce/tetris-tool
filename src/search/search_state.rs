use crate::state::{Board, Piece, Rotation};

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

    pub fn successors(&self, board: &Board) -> Vec<Self> {
        vec![
            self.left(board),
            self.das_left(board),
            self.right(board),
            self.das_right(board),
            self.drop(board),
            self.rotate_cw(board),
            self.rotate_ccw(board),
            self.rotate_180(board),
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
        new_state.apply_kicks(board, self.rotation);
        new_state
    }

    fn apply_kicks(&mut self, board: &Board, old_rot: Rotation) {
        let original = self.clone();
        for &(kick_row, kick_col) in self.piece.kick_map(old_rot, self.rotation).iter() {
            self.row += kick_row;
            self.col += kick_col;
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
}
