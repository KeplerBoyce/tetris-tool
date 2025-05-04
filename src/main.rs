use tetris::state::Game;
use tetris::logic::{Config, KeyAction, Stats};
use macroquad::prelude::*;
use tetris::ui::*;

#[macroquad::main("Tetris Program")]
async fn main() {
    let font = load_ttf_font("res/font.ttf").await.unwrap();
    let mut game = Game::new();
    let mut config = Config::default();
    let mut stats = Stats::new();
    let mut waiting_for_keybind: Option<KeyAction> = None;
    let mut keycode_set: Option<KeyCode> = None;
    let mut waiting = false;

    loop {
        clear_background(BLACK);
        draw_ui(&mut config, &mut waiting_for_keybind, &mut waiting);
        wait_for_keybind(&mut config, &mut waiting_for_keybind, &mut keycode_set);
        game.step(&config, &mut stats, waiting);
        game.draw(font, &stats);
        check_done_waiting(&mut waiting, &mut keycode_set);
        next_frame().await
    }
}
