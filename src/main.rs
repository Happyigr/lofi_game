mod animation;
mod camera;
mod constants;
mod enemy;
mod player;
mod resources;
mod sounds;
mod ui;

use animation::execute_animations;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use camera::spawn_camera;
use enemy::systems::{
    on_add_cathchable, on_enemy_kill, on_remove_cathchable, spawn_enemies, EnemyKilled,
};
use player::systems::{
    check_collision_with_enemy, move_player, on_player_upgrades, spawn_player, try_to_kill_enemy,
    PlayerUpgrade,
};
use resources::{init_materials, Game, GameState, Materials, MySettings};
use sounds::{change_audio, start_bg_audio};
use ui::{change_score, spawn_info_window, spawn_menu, spawn_score};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, EguiPlugin, AudioPlugin));
    app.init_resource::<Materials>();
    app.init_resource::<MySettings>();
    app.insert_resource(Game::default());
    app.init_state::<GameState>();
    app.add_event::<EnemyKilled>();
    app.add_event::<PlayerUpgrade>();
    app.add_systems(PreStartup, init_materials);

    app.add_systems(Startup, (start_bg_audio, spawn_camera));
    app.add_systems(
        Update,
        (change_audio).run_if(resource_changed::<MySettings>),
    );

    app.add_systems(Update, spawn_info_window);

    app.add_systems(Update, (spawn_menu).run_if(in_state(GameState::Menu)));

    // app.add_systems(OnEnter(GameState::Menu), todo!());
    app.add_systems(
        OnEnter(GameState::Game),
        (spawn_player, spawn_enemies, spawn_score),
    );

    app.add_systems(
        Update,
        (
            check_collision_with_enemy,
            try_to_kill_enemy,
            move_player,
            execute_animations,
            change_score,
            on_enemy_kill,
            on_player_upgrades,
        )
            .run_if(in_state(GameState::Game)),
    );

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);

    app.run();
}
