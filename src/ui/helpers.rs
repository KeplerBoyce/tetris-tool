use egui_macroquad::egui;
use crate::logic::Config;
use super::lock_game_x;

pub fn draw_ui(config: &mut Config) {
    egui_macroquad::ui(|egui_ctx| {
        egui::SidePanel::left("game_settings")
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.heading("Game Settings");
                ui.add(egui::Slider::new(&mut config.gravity, 0.0..=30.0).text("Gravity (units/s)"));
                ui.add(egui::Slider::new(&mut config.das, 0..=500).text("DAS (ms)"));
                ui.add(egui::Slider::new(&mut config.arr, 0..=50).text("ARR (ms/unit)"));
                ui.add(egui::Slider::new(&mut config.sdr, 0..=50).text("SDR (ms/unit)"));
            });
        *lock_game_x() = egui_ctx.used_rect().right();
    });
    egui_macroquad::draw();
}
