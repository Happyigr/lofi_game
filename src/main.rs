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
use resources::{init_materials, Game, Materials};
use sounds::spawn_heartbeat_bg_sound;
use ui::{change_score, spawn_info_window, spawn_score};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, EguiPlugin, AudioPlugin));
    app.insert_resource(Game::default());
    app.add_event::<EnemyKilled>();
    app.add_event::<PlayerUpgrade>();
    app.init_resource::<Materials>();
    app.add_systems(PreStartup, init_materials);

    app.add_systems(
        Startup,
        (
            spawn_camera,
            spawn_player,
            spawn_enemies,
            spawn_score,
            spawn_heartbeat_bg_sound,
        ),
    );

    app.add_systems(Update, execute_animations);
    app.add_systems(Update, spawn_info_window);
    app.add_systems(Update, change_score);
    app.add_systems(
        Update,
        (check_collision_with_enemy, try_to_kill_enemy, move_player),
    );
    app.add_systems(Update, (on_enemy_kill, on_player_upgrades));

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);

    app.run();
}
