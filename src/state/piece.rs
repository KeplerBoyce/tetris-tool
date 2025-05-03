use macroquad::prelude::*;
use crate::util::window::tile_size;
use super::Rotation;

#[derive(Clone, Copy)]
pub enum Piece {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::I => Color::new(0.0, 1.0, 1.0, 1.0),
            Piece::J => BLUE,
            Piece::L => ORANGE,
            Piece::O => YELLOW,
            Piece::S => GREEN,
            Piece::T => MAGENTA,
            Piece::Z => RED,
        }
    }

    pub fn draw(&self, x: f32, y: f32, scale: f32) -> (f32, f32) {
        let unit = tile_size() * scale;

        match self {
            Piece::I => {
                draw_rectangle(x, y, 4.0 * unit, unit, self.color());
                (4.0 * unit, unit)
            },
            Piece::J => {
                draw_rectangle(x, y, unit, unit, self.color());
                draw_rectangle(x, y + unit, 3.0 * unit, unit, self.color());
                (3.0 * unit, 2.0 * unit)
            },
            Piece::L => {
                draw_rectangle(x + 2.0 * unit, y, unit, unit, self.color());
                draw_rectangle(x, y + unit, 3.0 * unit, unit, self.color());
                (3.0 * unit, 2.0 * unit)
            },
            Piece::O => {
                draw_rectangle(x, y, 2.0 * unit, 2.0 * unit, self.color());
                (2.0 * unit, 2.0 * unit)
            },
            Piece::S => {
                draw_rectangle(x + unit, y, 2.0 * unit, unit, self.color());
                draw_rectangle(x, y + unit, 2.0 * unit, unit, self.color());
                (3.0 * unit, 2.0 * unit)
            },
            Piece::T => {
                draw_rectangle(x + unit, y, unit, unit, self.color());
                draw_rectangle(x, y + unit, 3.0 * unit, unit, self.color());
                (3.0 * unit, 2.0 * unit)
            },
            Piece::Z => {
                draw_rectangle(x, y, 2.0 * unit, unit, self.color());
                draw_rectangle(x + unit, y + unit, 2.0 * unit, unit, self.color());
                (3.0 * unit, 2.0 * unit)
            },
        }
    }

    pub fn offset_map(&self, rotation: Rotation) -> [(i8, i8); 4] {
        match self {
            Piece::I => {
                match rotation {
                    Rotation::Normal => [(0, -1), (0, 0), (0, 1), (0, 2)],
                    Rotation::Cw => [(-1, 0), (0, 0), (1, 0), (2, 0)],
                    Rotation::Ccw => [(-2, 0), (-1, 0), (0, 0), (1, 0)],
                    Rotation::Flip => [(0, -2), (0, -1), (0, 0), (0, 1)],
                }
            }
            Piece::J => {
                match rotation {
                    Rotation::Normal => [(-1, -1), (0, -1), (0, 0), (0, 1)],
                    Rotation::Cw => [(-1, 1), (-1, 0), (0, 0), (1, 0)],
                    Rotation::Ccw => [(-1, 0), (0, 0), (1, 0), (1, -1)],
                    Rotation::Flip => [(0, -1), (0, 0), (0, 1), (1, 1)],
                }
            }
            Piece::L => {
                match rotation {
                    Rotation::Normal => [(-1, 1), (0, -1), (0, 0), (0, 1)],
                    Rotation::Cw => [(1, 1), (-1, 0), (0, 0), (1, 0)],
                    Rotation::Ccw => [(-1, 0), (0, 0), (1, 0), (-1, -1)],
                    Rotation::Flip => [(0, -1), (0, 0), (0, 1), (1, -1)],
                }
            }
            Piece::O => {
                [(0, 0), (0, 1), (1, 0), (1, 1)]
            }
            Piece::S => {
                match rotation {
                    Rotation::Normal => [(-1, 0), (-1, 1), (0, 0), (0, -1)],
                    Rotation::Cw => [(-1, 0), (0, 0), (0, 1), (1, 1)],
                    Rotation::Ccw => [(-1, -1), (0, -1), (0, 0), (1, 0)],
                    Rotation::Flip => [(0, 0), (0, 1), (1, -1), (1, 0)],
                }
            }
            Piece::T => {
                match rotation {
                    Rotation::Normal => [(-1, 0), (0, -1), (0, 0), (0, 1)],
                    Rotation::Cw => [(-1, 0), (0, 0), (0, 1), (1, 0)],
                    Rotation::Ccw => [(-1, 0), (0, -1), (0, 0), (1, 0)],
                    Rotation::Flip => [(0, -1), (0, 0), (0, 1), (1, 0)],
                }
            }
            Piece::Z => {
                match rotation {
                    Rotation::Normal => [(-1, 0), (-1, -1), (0, 0), (0, 1)],
                    Rotation::Cw => [(-1, 1), (0, 0), (0, 1), (1, 0)],
                    Rotation::Ccw => [(-1, 0), (0, -1), (0, 0), (1, -1)],
                    Rotation::Flip => [(0, 0), (0, -1), (1, 1), (1, 0)],
                }
            }
        }
    }
}
