use std::fmt::Debug;
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

    pub fn is_empty(&self) -> bool {
        for r in 3..23 {
            for c in 0..10 {
                if self.tiles[r][c].piece.is_some() {
                    return false;
                }
            }
        }
        true
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

    // Returns the number of lines cleared
    pub fn clear_lines(&mut self) -> u8 {
        // Handle clearing lines
        let mut cleared = [false; 23];
        'row: for r in 0..23 {
            for c in 0..10 {
                if self.tiles[r][c].piece.is_none() {
                    continue 'row;
                }
            }
            // If we're here, this means that the row was completely full -- mark it to clear
            cleared[r] = true;
        }
        // Shift rows downwards to remove cleared lines
        let mut offset = 0;
        for r in (0..23).rev() {
            if cleared[r] {
                offset += 1;
                continue;
            }
            for c in 0..10 {
                self.tiles[r + offset][c] = self.tiles[r][c]
            }
        }
        // Finally, make sure to erase the top lines that didn't get overwritten by shift
        for r in 0..offset {
            for c in 0..10 {
                self.tiles[r + 3][c].piece = None;
            }
        }
        return offset as u8;
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..23 {
            for c in 0..10 {
                match self.tiles[r][c].piece {
                    Some(piece) => write!(f, "{:?}", piece)?,
                    None => write!(f, "-")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
