use macroquad::prelude::*;
use super::window::{text_size_large, text_size_normal, text_size_small};

pub fn text_small(font: Font, color: Color) -> TextParams {
    TextParams {
        font,
        font_size: text_size_small() as u16,
        color,
        ..Default::default()
    }
}

pub fn text_normal(font: Font, color: Color) -> TextParams {
    TextParams {
        font,
        font_size: text_size_normal() as u16,
        color,
        ..Default::default()
    }
}

pub fn text_large(font: Font, color: Color) -> TextParams {
    TextParams {
        font,
        font_size: text_size_large() as u16,
        color,
        ..Default::default()
    }
}
