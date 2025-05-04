use tetris::state::Game;
use tetris::logic::Config;
use macroquad::prelude::*;
use tetris::ui::*;

#[macroquad::main("Tetris Program")]
async fn main() {
    let font = load_ttf_font("res/inter.ttf").await.unwrap();
    let mut game = Game::new();
    let mut config = Config::default();

    loop {
        clear_background(BLACK);
        draw_ui(&mut config);
        game.step(&config);
        game.draw(font);
        next_frame().await
    }
}
