use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::resources::GameState;

pub fn spawn_menu(mut contexts: EguiContexts, mut game_state: ResMut<NextState<GameState>>) {
    egui::SidePanel::left("side_panel")
        .default_width(400.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.with_layout(
                egui::Layout {
                    main_dir: egui::Direction::TopDown,
                    main_wrap: false,
                    main_align: egui::Align::Min,
                    main_justify: false,
                    cross_align: egui::Align::Center,
                    cross_justify: false,
                },
                |ui| {
                    ui.add(egui::widgets::Image::new(egui::include_image!(
                        "../../assets/icon.png"
                    )));

                    ui.separator();

                    if ui.button("Start a game").clicked() {
                        game_state.set(GameState::Game);
                    }
                },
            )
        });
}
