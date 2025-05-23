use std::hash::{Hash, Hasher};
use macroquad::prelude::*;
use crate::util::window::*;
use crate::state::{Board, Piece};
use super::Placement;

#[derive(Clone, Debug)]
pub struct Pc {
    board: Board,
    placements: Vec<Placement>,
}

impl Pc {
    pub fn new(board: Board, placements: Vec<Placement>) -> Self {
        Self {
            board,
            placements,
        }
    }

    pub fn height(&self) -> u8 {
        let mut mino_count = 0;
        for r in 3..23 {
            for c in 0..10 {
                if self.board.tiles[r][c].piece.is_some() {
                    mino_count += 1;
                }
            }
        }
        for &placement in self.placements.iter() {
            if let Placement::Place{ .. } = placement {
                mino_count += 4;
            }
        }
        (mino_count / 10) as u8
    }

    fn get_final_board(&self) -> (Board, usize) {
        // Create a board containing all pieces in the PC
        let mut final_board = Board::new();
        let mut temp_board = self.board;
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
        (final_board, cleared)
    }

    // Returns the total height of the drawn PC
    pub fn draw(&self, x: f32, y: f32, scale: f32) -> f32 {
        let (final_board, cleared) = self.get_final_board();
        // Draw the board
        for r in (23 - cleared)..23 {
            for c in 0..10 {
                // If the piece has already been placed, draw as gray to make solution easier to read
                if self.board.tiles[r][c].piece.is_some() {
                    draw_rectangle(
                        x + tile_size() * scale * c as f32,
                        y + tile_size() * scale * (r - (23 - cleared)) as f32,
                        tile_size() * scale,
                        tile_size() * scale,
                        GRAY,
                    );
                    continue;
                }
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

    // Draws the order that the pieces are placed, returns height of drawing
    pub fn draw_sequence(&self, x: f32, y: f32, scale: f32) -> f32 {
        let tile = tile_size() * scale;
        let margin = tile_size() * 0.25;
        let mut width: f32 = margin;

        for &placement in self.placements.iter() {
            match placement {
                Placement::Hold => {},
                Placement::Place { piece, .. } => {
                    match piece {
                        Piece::I => {
                            draw_rectangle(x + width, y + 0.5 * tile, 4.0 * tile, tile, Piece::I.color());
                            width += 4.0 * tile + margin;
                        },
                        Piece::J => {
                            draw_rectangle(x + width, y, tile, tile, Piece::J.color());
                            draw_rectangle(x + width, y + tile, 3.0 * tile, tile, Piece::J.color());
                            width += 3.0 * tile + margin;
                        },
                        Piece::L => {
                            draw_rectangle(x + width + 2.0 * tile, y, tile, tile, Piece::L.color());
                            draw_rectangle(x + width, y + tile, 3.0 * tile, tile, Piece::L.color());
                            width += 3.0 * tile + margin;
                        },
                        Piece::O => {
                            draw_rectangle(x + width, y, 2.0 * tile, 2.0 * tile, Piece::O.color());
                            width += 2.0 * tile + margin;
                        },
                        Piece::S => {
                            draw_rectangle(x + width + tile, y, 2.0 * tile, tile, Piece::S.color());
                            draw_rectangle(x + width, y + tile, 2.0 * tile, tile, Piece::S.color());
                            width += 3.0 * tile + margin;
                        },
                        Piece::T => {
                            draw_rectangle(x + width + tile, y, tile, tile, Piece::T.color());
                            draw_rectangle(x + width, y + tile, 3.0 * tile, tile, Piece::T.color());
                            width += 3.0 * tile + margin;
                        },
                        Piece::Z => {
                            draw_rectangle(x + width, y, 2.0 * tile, tile, Piece::Z.color());
                            draw_rectangle(x + width + tile, y + tile, 2.0 * tile, tile, Piece::Z.color());
                            width += 3.0 * tile + margin;
                        },
                    }
                },
            }
        }
        2.0 * tile
    }
}

// Custom PartialEq and Hash implementations for avoiding duplicate PC solves
impl PartialEq for Pc {
    fn eq(&self, other: &Self) -> bool {
        self.get_final_board() == other.get_final_board()
    }
}

impl Eq for Pc {}

impl Hash for Pc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_final_board().hash(state);
    }
}
