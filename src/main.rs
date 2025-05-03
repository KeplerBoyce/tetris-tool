use tetris::state::Game;
use tetris::logic::Config;
use macroquad::prelude::*;

#[macroquad::main("Tetris Program")]
async fn main() {
    let mut game = Game::new();
    let config = Config::default();

    loop {
        clear_background(BLACK);
        game.step(&config);
        game.draw();
        next_frame().await
    }
}
