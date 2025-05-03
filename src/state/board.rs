use macroquad::prelude::*;
use crate::util::window::*;
use crate::state::Tile;

pub struct Board {
    pub tiles: [[Tile; 10]; 20],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::new(); 10]; 20],
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        // Horizontal grid lines
        for r in 0..=20 {
            draw_line(
                x,
                y + r as f32 * tile_size(),
                x + board_width(),
                y + r as f32 * tile_size(),
                GRID_THICKNESS,
                if r == 0 || r == 20 {
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
                y + GRID_THICKNESS / 2.0,
                x + c as f32 * tile_size(),
                y + board_height() - GRID_THICKNESS / 2.0,
                GRID_THICKNESS,
                if c == 0 || c == 10 {
                    WHITE
                } else {
                    Color::new(0.2, 0.2, 0.2, 1.0)
                },
            );
        }
        // Pieces
        for r in 0..20 {
            for c in 0..10 {
                draw_rectangle(
                    x + c as f32 * tile_size() + GRID_THICKNESS / 2.0,
                    y + r as f32 * tile_size() + GRID_THICKNESS / 2.0,
                    tile_size() - GRID_THICKNESS,
                    tile_size() - GRID_THICKNESS,
                    self.tiles[r][c].color(),
                );
            }
        }
    }
}
