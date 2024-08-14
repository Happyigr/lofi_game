mod animation;
mod assets;
mod camera;
mod constants;
mod enemy;
mod player;

use animation::execute_animations;
use bevy::prelude::*;
use camera::spawn_camera;
use enemy::systems::{on_add_cathchable, on_enemy_despawn, on_remove_cathchable, spawn_enemies};
use player::systems::{check_collision_with_enemy, move_player, spawn_player, try_to_kill_enemy};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies));
    app.add_systems(Update, execute_animations);
    app.add_systems(
        Update,
        (check_collision_with_enemy, try_to_kill_enemy, move_player).chain(),
    );

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);
    app.observe(on_enemy_despawn);
    // app.observe(on_boom_animation_pending);

    app.run();
}
