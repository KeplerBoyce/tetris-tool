use std::collections::VecDeque;
use crate::state::{Piece, Rotation};
use crate::logic::init_queue;
use crate::util::window::*;
use super::Board;

pub struct Game {
    pub board: Board,
    pub piece: Option<Piece>,
    pub hold: Option<Piece>,
    pub queue: VecDeque<Piece>,
    pub row: u8,
    pub col: u8,
    pub rotation: Rotation,
    pub bag: VecDeque<Piece>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            piece: None,
            hold: None,
            queue: VecDeque::new(),
            row: 0,
            col: 0,
            rotation: Rotation::Normal,
            bag: VecDeque::new(),
        };
        init_queue(&mut game);
        game
    }

    pub fn draw(&self) {
        self.board.draw(board_x(), board_y());
        self.draw_queue(queue_x(), board_y(), 0.5);
    }

    fn draw_queue(&self, x: f32, y: f32, scale: f32) {
        let mut height: f32 = 0.0;
        for &piece in self.queue.iter() {
            let (_, h) = piece.draw(x, y + height, scale);
            height += h + QUEUE_GAP;
        }
    }
}
