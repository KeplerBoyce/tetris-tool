use tetris::state::Game;
use macroquad::prelude::*;

#[macroquad::main("Tetris Program")]
async fn main() {
    let game = Game::new();

    loop {
        clear_background(BLACK);
        game.draw();
        next_frame().await
    }
}
