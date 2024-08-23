use crate::{
    enemy::SuperPower,
    resources::{Game, MySettings},
};
use bevy::{color::ColorToPacked, prelude::ResMut};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};
use strum::IntoEnumIterator;

pub fn spawn_info_window(mut contexts: EguiContexts, mut settings: ResMut<MySettings>) {
    egui::Window::new("Info").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout {
                main_dir: egui::Direction::TopDown,
                main_wrap: false,
                main_align: egui::Align::Center,
                main_justify: true,
                cross_align: egui::Align::Center,
                cross_justify: false,
            },
            |ui| {
                ui.horizontal_top(|ui| {
                    ui.label("Type");
                    ui.label(" | ");
                    ui.label("Kill Key");
                });

                for superpower in SuperPower::iter() {
                    ui.horizontal_top(|ui| {
                        let color = superpower.get_enemy_color().to_linear().to_u8_array();
                        ui.colored_label(
                            Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]),
                            format!("{superpower}"),
                        );
                        ui.label(" | ");
                        ui.label(format!("{}", superpower.get_keycode_str()));
                    });
                }

                ui.horizontal_top(|ui| {
                    ui.add(
                        egui::Slider::new(&mut settings.boom_anim_fps, 10..=60)
                            .text("Boom anim FPS"),
                    );
                });
                ui.horizontal_top(|ui| {
                    ui.checkbox(&mut settings.audio_on, "Audio");
                    ui.add(
                        egui::Slider::new(&mut settings.bg_sound_volume, 0.0..=1.)
                            .text("Master Volume"),
                    );
                });
            },
        );
    });
}
