use macroquad::prelude::*;
use crate::util::window::*;
use crate::state::Tile;
use super::{Piece, Rotation};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    pub tiles: [[Tile; 10]; 23],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::new(); 10]; 23],
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        for r in 0..23 {
            for c in 0..10 {
                draw_rectangle(
                    x + c as f32 * tile_size() + grid_thickness() / 2.0,
                    y + r as f32 * tile_size() + grid_thickness() / 2.0,
                    tile_size() - grid_thickness(),
                    tile_size() - grid_thickness(),
                    self.tiles[r][c].color(),
                );
            }
        }
    }

    pub fn draw_grid(&self, x: f32, y: f32) {
        // Horizontal grid lines
        for r in 3..=23 {
            draw_line(
                x,
                y + r as f32 * tile_size(),
                x + board_width(),
                y + r as f32 * tile_size(),
                grid_thickness(),
                if r == 3 || r == 23 {
                    WHITE
                } else {
                    Color::new(0.2, 0.2, 0.2, 1.0)
                },
            );
        }
        // Vertical grid lines
        for c in 0..=10 {
            draw_line(
                x + c as f32 * tile_size(),
                y + 3.0 * tile_size() + grid_thickness() / 2.0,
                x + c as f32 * tile_size(),
                y + board_height() - grid_thickness() / 2.0,
                grid_thickness(),
                if c == 0 || c == 10 {
                    WHITE
                } else {
                    Color::new(0.2, 0.2, 0.2, 1.0)
                },
            );
        }
    }

    pub fn with_placement(&self, piece: Piece, row: u8, col: u8, rotation: Rotation) -> Self {
        let mut new_board = *self;
        for &(offset_row, offset_col) in piece.offset_map(rotation).iter() {
            let r = row as i8 + offset_row;
            let c = col as i8 + offset_col;
            new_board.tiles[r as usize][c as usize] = Tile::from(piece);
        }
        new_board
    }
}
