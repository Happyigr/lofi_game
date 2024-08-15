use crate::enemy::SuperPower;
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;

pub fn spawn_info_window(mut contexts: EguiContexts) {
    egui::Window::new("Info").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label("Type | Kill Key");
            for superpower in SuperPower::iter() {
                ui.label(format!("{} | {}", superpower, superpower.get_keycode_str()));
            }
        });
    });
}
