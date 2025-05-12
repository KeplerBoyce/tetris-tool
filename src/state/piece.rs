use macroquad::prelude::*;
use crate::util::window::tile_size;
use super::Rotation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
            Piece::I => Color::from_hex(0x4dcdd1),
            Piece::J => Color::from_hex(0x4d70d1),
            Piece::L => Color::from_hex(0xdb9b4d),
            Piece::O => Color::from_hex(0xdbcd4d),
            Piece::S => Color::from_hex(0x63e06f),
            Piece::T => Color::from_hex(0xdd87e0),
            Piece::Z => Color::from_hex(0xe06363),
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

    pub fn kick_map(&self, old_rot: Rotation, new_rot: Rotation) -> Vec<(i8, i8)> {
        match self {
            Piece::I => {
                match (old_rot, new_rot) {
                    (Rotation::Normal, Rotation::Normal) => vec![(0, 0)],
                    (Rotation::Normal, Rotation::Cw) => vec![(0, 0), (0, -2), (0, 1), (1, -2), (-1, 2)],
                    (Rotation::Normal, Rotation::Ccw) => vec![(0, 0), (0, -1), (0, 2), (-2, -1), (-2, 1)],
                    (Rotation::Normal, Rotation::Flip) => vec![(0, 0)],
                    (Rotation::Cw, Rotation::Normal) => vec![(0, 0), (0, 1), (0, -2), (2, 1), (-1, 2)],
                    (Rotation::Cw, Rotation::Cw) => vec![(0, 0)],
                    (Rotation::Cw, Rotation::Ccw) => vec![(0, 0)],
                    (Rotation::Cw, Rotation::Flip) => vec![(0, 0), (0, -1), (0, 2), (-2, -1), (1, 2)],
                    (Rotation::Ccw, Rotation::Normal) => vec![(0, 0), (0, 1), (0, -2), (2, 1), (-1, -2)],
                    (Rotation::Ccw, Rotation::Cw) => vec![(0, 0)],
                    (Rotation::Ccw, Rotation::Ccw) => vec![(0, 0)],
                    (Rotation::Ccw, Rotation::Flip) => vec![(0, 0), (0, -2), (0, 1), (1, -2), (-2, 1)],
                    (Rotation::Flip, Rotation::Normal) => vec![(0, 0)],
                    (Rotation::Flip, Rotation::Cw) => vec![(0, 0), (0, 1), (0, -2), (2, 1), (-1, -2)],
                    (Rotation::Flip, Rotation::Ccw) => vec![(0, 0), (0, 2), (0, -1), (-1, 2), (2, -1)],
                    (Rotation::Flip, Rotation::Flip) => vec![(0, 0)],
                }
            }
            Piece::J | Piece::L | Piece::S | Piece::T | Piece::Z => {
                match (old_rot, new_rot) {
                    (Rotation::Normal, Rotation::Normal) => vec![(0, 0)],
                    (Rotation::Normal, Rotation::Cw) => vec![(0, 0), (0, -1), (-1, -1), (2, 0), (2, -1)],
                    (Rotation::Normal, Rotation::Ccw) => vec![(0, 0), (0, 1), (-1, 1), (2, 0), (2, 1)],
                    (Rotation::Normal, Rotation::Flip) => vec![(0, 0), (-1, 0), (-1, 1), (-1, -1), (0, 1), (0, -1)],
                    (Rotation::Cw, Rotation::Normal) => vec![(0, 0), (0, 1), (1, 1), (-2, 0), (-2, 1)],
                    (Rotation::Cw, Rotation::Cw) => vec![(0, 0)],
                    (Rotation::Cw, Rotation::Ccw) => vec![(0, 0), (0, 1), (-2, 1), (-1, 1), (-2, 0), (-1, 0)],
                    (Rotation::Cw, Rotation::Flip) => vec![(0, 0), (0, 1), (1, 1), (-2, 0), (-2, 1)],
                    (Rotation::Ccw, Rotation::Normal) => vec![(0, 0), (0, -1), (1, -1), (-2, 0), (-2, -1)],
                    (Rotation::Ccw, Rotation::Cw) => vec![(0, 0), (0, -1), (-2, -1), (-1, -1), (-2, 0), (-1, 0)],
                    (Rotation::Ccw, Rotation::Ccw) => vec![(0, 0)],
                    (Rotation::Ccw, Rotation::Flip) => vec![(0, 0), (0, -1), (1, -1), (-2, 0), (-2, -1)],
                    (Rotation::Flip, Rotation::Normal) => vec![(0, 0), (1, 0), (1, -1), (1, 1), (0, -1), (0, 1)],
                    (Rotation::Flip, Rotation::Cw) => vec![(0, 0), (0, -1), (-1, -1), (2, 0), (2, -1)],
                    (Rotation::Flip, Rotation::Ccw) => vec![(0, 0), (0, 1), (-1, 1), (2, 0), (2, 1)],
                    (Rotation::Flip, Rotation::Flip) => vec![(0, 0)],
                }
            }
            Piece::O => {
                vec![(0, 0)]
            }
        }
    }
}
