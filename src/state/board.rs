use macroquad::prelude::*;
use crate::util::window::*;
use crate::state::Tile;

#[derive(Clone, Copy)]
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
}
