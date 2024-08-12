mod bundles;
mod components;
mod constants;
mod observers;
mod start_systems;
mod update_systems;

use bevy::prelude::*;
use components::*;
use observers::*;
use start_systems::*;
use update_systems::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies));
    app.add_systems(Update, (check_collison_with_radius, execute_animations));

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);
    app.observe(on_enemy_despawn);
    // app.observe(on_boom_animation_pending);

    app.run();
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimConfig, &mut TextureAtlas)>,
) {
    for (entity, mut config, mut atlas) in &mut query {
        dbg!("smth");
        // we track how long the current sprite has been displayed for
        config.timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.timer.just_finished() {
            // If it has been displayed for the user-defined amount of time (fps)...
            if atlas.index == config.last_i {
                // ...and it IS the last frame, then we despawn the animation
                commands.entity(entity).despawn();
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.timer = AnimConfig::timer_from_fps(config.fps);
            }
        }
    }
}
