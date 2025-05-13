use crate::state::{Piece, Rotation};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Placement {
    Hold,
    Place {
        piece: Piece,
        row: u8,
        col: u8,
        rotation: Rotation,
    },
}

impl Placement {
    pub fn place(piece: Piece, row: u8, col: u8, rotation: Rotation) -> Self {
        Self::Place {
            piece,
            row,
            col,
            rotation,
        }
    }
}
