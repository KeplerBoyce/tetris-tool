use macroquad::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;
use crate::search::get_finesse_faults;
use crate::state::{Piece, Rotation};
use crate::logic::*;
use crate::util::font::*;
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
    pub last_time: Instant, // Timestamp of when last gravity falling unit occurred
    pub left_time: Instant, // Timestamp of when last left DAS unit occurred
    pub right_time: Instant, // Timestamp of when last right DAS unit occurred
    pub soft_drop_time: Instant, // Timestamp of when last softdrop unit occurred
    pub left_das_activated: bool, // Becomes true when left key is held long enough for DAS
    pub right_das_activated: bool, // Becomes true when right key is held long enough for DAS
    pub left_priority: bool, // True when left is the most recently held key
    pub undo_stack: Vec<(Board, Option<Piece>, Stats)>,
    pub prev_stats: Stats,
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
            last_time: Instant::now(),
            left_time: Instant::now(),
            right_time: Instant::now(),
            soft_drop_time: Instant::now(),
            left_das_activated: false,
            right_das_activated: false,
            left_priority: false,
            undo_stack: vec![],
            prev_stats: Stats::new(),
        };
        init_queue(&mut game);
        game
    }

    pub fn draw(&mut self, font: Font, stats: &Stats) {
        self.board.draw(board_x(), board_y());
        self.draw_piece(board_x(), board_y());
        self.draw_shadow(board_x(), board_y());
        self.draw_queue(queue_x(), queue_y(), 0.75, font);
        self.draw_hold(hold_x(), hold_y(), 0.75, font);
        self.draw_stats(finesse_x(), finesse_y(), font, stats);
        self.board.draw_grid(board_x(), board_y());
        Game::draw_borders();
    }

    fn draw_borders() {
        fn draw_outline(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) {
            let x1 = x - thickness / 2.0;
            let x2 = x1 + w;
            let y1 = y - thickness / 2.0;
            let y2 = y1 + h;
            draw_rectangle(x1, y1, w + thickness, thickness, color);
            draw_rectangle(x1, y1, thickness, h + thickness, color);
            draw_rectangle(x1, y2, w + thickness, thickness, color);
            draw_rectangle(x2, y1, thickness, h + thickness, color);
        }
        // Board outline
        draw_outline(board_x(), board_y(), board_width(), board_height(), grid_thickness(), WHITE);
        // Hold outline
        draw_outline(hold_x(), hold_y(), hold_width(), hold_height(), grid_thickness(), WHITE);
        // Finesse outline
        draw_outline(finesse_x(), finesse_y(), finesse_width(), finesse_height(), grid_thickness(), WHITE);
        // Queue outline
        draw_outline(queue_x(), queue_y(), hold_width(), queue_height(), grid_thickness(), WHITE);
    }

    fn draw_queue(&self, x: f32, y: f32, scale: f32, font: Font) {
        draw_text_ex("QUEUE", x + margin(), y + tile_size(), text_large(font, WHITE));
        let mut height: f32 = text_size_large() + 2.0 * margin();
        // Draw only the first 5 pieces in queue in case we undid moves
        for &piece in self.queue.iter().take(5) {
            let (_, h) = piece.draw(x + margin(), y + height, scale);
            height += h + queue_gap();
        }
    }

    fn draw_hold(&self, x: f32, y: f32, scale: f32, font: Font) {
        draw_text_ex("HOLD", x + margin(), y + tile_size(), text_large(font, WHITE));
        if let Some(hold) = self.hold {
            hold.draw(x + margin(), y + text_size_large() + 2.0 * margin(), scale);
        }
    }

    fn draw_piece(&self, x: f32, y: f32) {
        if let Some(piece) = self.piece {
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                draw_rectangle(
                    x + (self.piece_col + offset_col) as f32 * tile_size() + grid_thickness() / 2.0,
                    y + (self.piece_row + offset_row) as f32 * tile_size() + grid_thickness() / 2.0,
                    tile_size() - grid_thickness(),
                    tile_size() - grid_thickness(),
                    piece.color(),
                );
            }
        }
    }

    fn draw_shadow(&mut self, x: f32, y: f32) {
        if let Some(piece) = self.piece {
            let old_piece_row = self.piece_row;
            loop {
                self.piece_row += 1;
                if self.check_landing() {
                    self.piece_row -= 1;
                    break;
                }
            }
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                let mut color = piece.color();
                color.a = 0.5;
                draw_rectangle(
                    x + (self.piece_col + offset_col) as f32 * tile_size() + grid_thickness() / 2.0,
                    y + (self.piece_row + offset_row) as f32 * tile_size() + grid_thickness() / 2.0,
                    tile_size() - grid_thickness(),
                    tile_size() - grid_thickness(),
                    color,
                );
            }
            self.piece_row = old_piece_row;
        }
    }

    fn draw_stats(&self, x: f32, y: f32, font: Font, stats: &Stats) {
        draw_text_ex("STATS", x + margin(), y + tile_size(), text_large(font, WHITE));
        draw_text_ex(&format!("Pieces: {}", stats.pieces), x + margin(),
                y + text_size_large() + 2.0 * margin(), text_normal(font, WHITE));
        draw_text_ex(&format!("Lines: {}", stats.lines), x + margin(),
                y + text_size_large() + 2.0 * margin() + text_size_normal(), text_normal(font, WHITE));
        draw_text_ex(&format!("Inputs: {}", stats.inputs), x + margin(),
                y + text_size_large() + 2.0 * margin() + 2.0 * text_size_normal(), text_normal(font, WHITE));
        draw_text_ex(&format!("Faults: {}", stats.faults), x + margin(),
                y + text_size_large() + 2.0 * margin() + 3.0 * text_size_normal(), text_normal(font, WHITE));
    }

    pub fn refresh_last_time(&mut self) {
        self.last_time = Instant::now();
    }

    fn apply_gravity(&mut self, config: &Config, stats: &mut Stats) {
        let now = Instant::now();
        let fall_time = now.duration_since(self.last_time).as_millis() as u32;
        // Check if we have already touched the ground -- wait until soft drop time elapses
        if config.gravity > 0.0 {
            self.piece_row += 1;
            if self.check_landing() {
                if fall_time > config.grace_period {
                    self.piece_row -= 1;
                    self.place_piece(stats);
                } else {
                    self.piece_row -= 1;
                    return;
                }
            }
            self.piece_row -= 1;
        }
        // Otherwise, apply gravity
        if fall_time >= (1000.0 / config.gravity) as u32 {
            self.piece_row += 1;
            self.refresh_last_time();
            if self.check_landing() {
                self.piece_row -= 1;
                self.place_piece(stats);
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

    pub fn place_piece(&mut self, stats: &mut Stats) {
        let moves = stats.inputs - self.prev_stats.inputs;
        if let Some(piece) = self.piece {
            stats.faults += get_finesse_faults(&self.board, piece, moves as u8, self.piece_row as u8, self.piece_col as u8, self.rotation) as u32;
        }
        stats.pieces += 1;
        self.undo_stack.push((self.board, self.piece, self.prev_stats));
        self.prev_stats = *stats;
        if let Some(piece) = self.piece {
            for &(offset_row, offset_col) in piece.offset_map(self.rotation).iter() {
                let row = (self.piece_row + offset_row) as usize;
                let col = (self.piece_col + offset_col) as usize;
                self.board.tiles[row][col] = Tile::from(piece);
                self.piece = None;
                self.rotation = Rotation::Normal;
            }
        }
        // Handle clearing lines
        let mut cleared = [false; 23];
        'row: for r in 0..23 {
            for c in 0..10 {
                if self.board.tiles[r][c].piece.is_none() {
                    continue 'row;
                }
            }
            // If we're here, this means that the row was completely full -- mark it to clear
            cleared[r] = true;
            stats.lines += 1;
        }
        // Shift rows downwards to remove cleared lines
        let mut offset = 0;
        for r in (0..23).rev() {
            if cleared[r] {
                offset += 1;
                continue;
            }
            for c in 0..10 {
                self.board.tiles[r + offset][c] = self.board.tiles[r][c]
            }
        }
        // Finally, make sure to erase the top lines that didn't get overwritten by shift
        for r in 0..offset {
            for c in 0..10 {
                self.board.tiles[r + 3][c].piece = None;
            }
        }
    }

    pub fn step(&mut self, config: &Config, stats: &mut Stats) {
        if self.piece.is_none() {
            get_next_piece(self);
            self.piece_row = 1;
            self.piece_col = 4;
            self.refresh_last_time();
        }
        handle_input(config, stats, self);
        self.apply_gravity(config, stats);
    }
}
