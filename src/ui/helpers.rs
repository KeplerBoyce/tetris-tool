use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;
use strum::IntoEnumIterator;
use crate::logic::{Config, KeyAction};
use super::lock_game_x;

fn get_keybind_label(action: KeyAction) -> String {
    String::from(match action {
        KeyAction::Left => "Left",
        KeyAction::Right => "Right",
        KeyAction::SoftDrop => "Soft Drop",
        KeyAction::HardDrop => "Hard Drop",
        KeyAction::RotateCw => "Rotate CW",
        KeyAction::RotateCcw => "Rotate CCW",
        KeyAction::Rotate180 => "Rotate 180",
        KeyAction::Hold => "Hold",
        KeyAction::Reset => "Reset",
        KeyAction::Undo => "Undo",
    })
}

fn get_keybind_button_text(action: KeyAction, waiting_for: Option<KeyAction>, config: &Config) -> String {
    if let Some(wait) = waiting_for {
        if wait == action {
            return String::from("Press any key...");
        }
    }
    format!("{:?}", match action {
        KeyAction::Left => config.left,
        KeyAction::Right => config.right,
        KeyAction::SoftDrop => config.soft_drop,
        KeyAction::HardDrop => config.hard_drop,
        KeyAction::RotateCw => config.rotate_cw,
        KeyAction::RotateCcw => config.rotate_ccw,
        KeyAction::Rotate180 => config.rotate_180,
        KeyAction::Hold => config.hold,
        KeyAction::Reset => config.reset,
        KeyAction::Undo => config.undo,
    })
}

fn add_button(
    ui: &mut Ui,
    action: KeyAction,
    waiting_for: &mut Option<KeyAction>,
    waiting: &mut bool,
    config: &Config,
) {
    if ui.button(get_keybind_button_text(action, *waiting_for, config)).clicked()
            && waiting_for.is_none() {
        *waiting_for = Some(action);
        *waiting = true;
    }
}

pub fn draw_ui(config: &mut Config, waiting_for: &mut Option<KeyAction>, waiting: &mut bool) {
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_pixels_per_point((screen_width() / 1500.0).max(1.0));

        egui::SidePanel::left("game_settings")
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.heading("Game Settings");
                ui.add(egui::Slider::new(&mut config.gravity, 0.0..=30.0).text("Gravity (units/s)"));
                ui.add(egui::Slider::new(&mut config.das, 0..=500).text("DAS (ms)"));
                ui.add(egui::Slider::new(&mut config.arr, 0..=50).text("ARR (ms/unit)"));
                ui.add(egui::Slider::new(&mut config.sdr, 0..=50).text("SDR (ms/unit)"));

                ui.separator();

                ui.heading("Key Bindings");
                egui::Grid::new("keybind_table")
                    .striped(true)
                    .show(ui, |ui| {
                        for action in KeyAction::iter() {
                            ui.label(get_keybind_label(action));
                            add_button(ui, action, waiting_for, waiting, config);
                            ui.end_row();
                        }
                    });
            });
        *lock_game_x() = egui_ctx.used_rect().right();
    });
    egui_macroquad::draw();
}

pub fn wait_for_keybind(
    config: &mut Config,
    waiting_for: &mut Option<KeyAction>,
    keycode_set: &mut Option<KeyCode>,
) {
    if let Some(action) = waiting_for {
        if let Some(key) = get_last_key_pressed() {
            match action {
                KeyAction::Left => config.left = key,
                KeyAction::Right => config.right = key,
                KeyAction::SoftDrop => config.soft_drop = key,
                KeyAction::HardDrop => config.hard_drop = key,
                KeyAction::RotateCw => config.rotate_cw = key,
                KeyAction::RotateCcw => config.rotate_ccw = key,
                KeyAction::Rotate180 => config.rotate_180 = key,
                KeyAction::Hold => config.hold = key,
                KeyAction::Reset => config.reset = key,
                KeyAction::Undo => config.undo = key,
            }
            *waiting_for = None;
            *keycode_set = Some(key);
        }
    }
}

pub fn check_done_waiting(waiting: &mut bool, keycode_set: &mut Option<KeyCode>) {
    if let Some(key) = keycode_set {
        if is_key_released(*key) {
            *waiting = false;
            *keycode_set = None;
        }
    }
}
