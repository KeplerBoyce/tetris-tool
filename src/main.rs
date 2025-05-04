use tetris::state::Game;
use tetris::logic::{Config, Stats};
use macroquad::prelude::*;
use tetris::ui::*;

#[macroquad::main("Tetris Program")]
async fn main() {
    let font = load_ttf_font("res/font.ttf").await.unwrap();
    let mut game = Game::new();
    let mut config = Config::default();
    let mut stats = Stats::new();

    loop {
        clear_background(BLACK);
        draw_ui(&mut config);
        game.step(&config, &mut stats);
        game.draw(font, &stats);
        next_frame().await
    }
}
