use macroquad::prelude::*;
use crate::util::window::tile_size;

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
            Piece::I => WHITE,
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
}
