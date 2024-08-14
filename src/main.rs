mod animation;
mod assets;
mod camera;
mod constants;
mod enemy;
mod player;

use animation::execute_animations;
use assets::{init_materials, Materials};
use bevy::prelude::*;
use camera::spawn_camera;
use enemy::systems::{
    on_add_cathchable, on_enemy_kill, on_remove_cathchable, spawn_enemies, EnemyKilled,
};
use player::systems::{check_collision_with_enemy, move_player, spawn_player, try_to_kill_enemy};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_event::<EnemyKilled>();
    app.init_resource::<Materials>();

    app.add_systems(
        Startup,
        (spawn_camera, spawn_player, spawn_enemies, init_materials),
    );

    app.add_systems(Update, execute_animations);
    app.add_systems(
        Update,
        (check_collision_with_enemy, try_to_kill_enemy, move_player),
    );
    app.add_systems(Update, on_enemy_kill);

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);

    app.run();
}
