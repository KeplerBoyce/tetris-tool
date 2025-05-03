use rand::Rng;
use crate::rng;
use crate::state::{Game, Piece};

pub fn gen_bag(game: &mut Game) {
    let mut bag = vec![Piece::I, Piece::J, Piece::L, Piece::O, Piece::S, Piece::T, Piece::Z];
    for i in (0..7).rev() {
        let piece = bag.remove(rng().random_range(0..=i));
        game.bag.push_back(piece);
    }
}

pub fn get_next_piece(game: &mut Game) {
    game.piece = Some(game.queue.pop_front().expect("Game queue empty!"));
    if game.bag.len() == 0 {
        gen_bag(game);
    }
    game.queue.push_back(game.bag.pop_front().expect("Game bag empty!"));
}

pub fn init_queue(game: &mut Game) {
    gen_bag(game);
    for _ in 0..5 {
        game.queue.push_back(game.bag.pop_front().expect("Game bag empty!"));
    }
}
