use macroquad::prelude::*;
use crate::state::Piece;

#[derive(Clone, Copy)]
pub struct Tile {
    pub piece: Option<Piece>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            piece: None,
        }
    }

    pub fn from(piece: Piece) -> Self {
        Self {
            piece: Some(piece),
        }
    }

    pub fn color(&self) -> Color {
        if let Some(piece) = self.piece {
            piece.color()
        } else {
            BLACK
        }
    }
}
