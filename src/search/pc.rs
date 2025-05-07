use macroquad::prelude::*;
use crate::{state::Board, util::window::tile_size};
use super::Placement;

#[derive(Clone, Debug)]
pub struct Pc {
    placements: Vec<Placement>,
}

impl Pc {
    pub fn new(placements: Vec<Placement>) -> Self {
        Self {
            placements,
        }
    }

    // Returns the total height of the drawn PC
    pub fn draw(&self, board: &Board, x: f32, y: f32, scale: f32) -> f32 {
        // Create a board containing all pieces in the PC
        let mut final_board = Board::new();
        let mut temp_board = *board;
        // Used for keeping track of which rows in final_board the temp_board rows correspond to
        let mut line_stack: Vec<usize> = (0..23).collect();
        let mut cleared = 0;

        for &placement in self.placements.iter() {
            if let Placement::Place { piece, row, col, rotation } = placement {
                temp_board = temp_board.with_placement(piece, row, col, rotation);
                // Check if this cleared any lines -- if so, add to final_board
                'row: for r in cleared..23 {
                    for c in 0..10 {
                        if temp_board.tiles[r][c].piece.is_none() {
                            continue 'row;
                        }
                    }
                    // If we're here, then we should copy this row into final board
                    cleared += 1;
                    let line_stack_idx = line_stack.len() - (23 - r);
                    for c in 0..10 {
                        final_board.tiles[line_stack[line_stack_idx]][c] = temp_board.tiles[r][c];
                    }
                    // Adjust line map accordingly after this line clear
                    line_stack.remove(line_stack_idx);
                }
                temp_board.clear_lines();
            }
        }
        // Draw the board
        for r in (23 - cleared)..23 {
            for c in 0..10 {
                draw_rectangle(
                    x + tile_size() * scale * c as f32,
                    y + tile_size() * scale * (r - (23 - cleared)) as f32,
                    tile_size() * scale,
                    tile_size() * scale,
                    final_board.tiles[r][c].color(),
                );
            }
        }
        cleared as f32 * tile_size() * scale
    }
}
