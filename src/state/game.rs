use macroquad::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;
use crate::state::{Piece, Rotation};
use crate::logic::*;
use crate::util::window::*;
use super::{Board, Tile};

pub struct Game {
    pub board: Board,
    pub piece: Option<Piece>,
    pub piece_row: i8,
    pub piece_col: i8,
    pub hold: Option<Piece>,
    pub queue: VecDeque<Piece>,
    pub rotation: Rotation,
    pub bag: VecDeque<Piece>,
    pub gravity: f32, // Measured in blocks per second
    pub soft_drop_time: u32, // Milliseconds before gravity places piece that is touching floor
    last_time: Instant,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            piece: None,
            piece_row: 0,
            piece_col: 0,
            hold: None,
            queue: VecDeque::new(),
            rotation: Rotation::Normal,
            bag: VecDeque::new(),
            gravity: 1.0,
            soft_drop_time: 500,
            last_time: Instant::now(),
        };
        init_queue(&mut game);
        game
    }

    pub fn draw(&self) {
        self.board.draw(board_x(), board_y());
        self.draw_piece(board_x(), board_y());
        self.draw_queue(queue_x(), board_y(), 0.5);
        self.draw_hold(hold_x(), board_y(), 0.5);
        self.board.draw_grid(board_x(), board_y());
    }

    fn draw_queue(&self, x: f32, y: f32, scale: f32) {
        let mut height: f32 = 0.0;
        for &piece in self.queue.iter() {
            let (_, h) = piece.draw(x, y + height, scale);
            height += h + QUEUE_GAP;
        }
    }

    fn draw_hold(&self, x: f32, y: f32, scale: f32) {
        if let Some(hold) = self.hold {
            hold.draw(x, y, scale);
        }
    }

    fn draw_piece(&self, x: f32, y: f32) {
        if let Some(piece) = self.piece {
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                draw_rectangle(
                    x + (self.piece_col + offset_col) as f32 * tile_size() + GRID_THICKNESS / 2.0,
                    y + (self.piece_row + offset_row) as f32 * tile_size() + GRID_THICKNESS / 2.0,
                    tile_size() - GRID_THICKNESS,
                    tile_size() - GRID_THICKNESS,
                    piece.color(),
                );
            }
        }
    }

    pub fn refresh_last_time(&mut self) {
        self.last_time = Instant::now();
    }

    fn apply_gravity(&mut self) {
        let now = Instant::now();
        let fall_time = now.duration_since(self.last_time).as_millis() as u32;
        // Check if we have already touched the ground -- wait until soft drop time elapses
        self.piece_row += 1;
        if self.check_landing() {
            if fall_time > self.soft_drop_time {
                self.piece_row -= 1;
                self.place_piece();
            } else {
                self.piece_row -= 1;
                return;
            }
        }
        self.piece_row -= 1;
        // Otherwise, apply gravity
        if fall_time >= (1000.0 / self.gravity) as u32 {
            self.piece_row += 1;
            self.refresh_last_time();
            if self.check_landing() {
                self.piece_row -= 1;
                self.place_piece();
            }
        }
    }

    pub fn check_landing(&self) -> bool {
        if let Some(piece) = self.piece {
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                let row = (self.piece_row + offset_row) as usize;
                let col = (self.piece_col + offset_col) as usize;
                if row > 22 || self.board.tiles[row][col].piece.is_some() {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn check_wall_intersect(&self) -> bool {
        if let Some(piece) = self.piece {
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                let row = self.piece_row + offset_row;
                let col = self.piece_col + offset_col;
                if col < 0 || col > 9 || self.board.tiles[row as usize][col as usize].piece.is_some() {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn place_piece(&mut self) {
        if let Some(piece) = self.piece {
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                let row = (self.piece_row + offset_row) as usize;
                let col = (self.piece_col + offset_col) as usize;
                self.board.tiles[row][col] = Tile::from(piece);
                self.piece = None;
                self.rotation = Rotation::Normal;
            }
        }
    }

    pub fn step(&mut self, config: &Config) {
        if self.piece.is_none() {
            get_next_piece(self);
            self.piece_row = 1;
            self.piece_col = 4;
            self.refresh_last_time();
        }
        handle_input(config, self);
        self.apply_gravity();
    }
}
