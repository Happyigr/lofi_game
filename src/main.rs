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
    app.add_systems(Update, (check_collison_with_radius,));

    app.observe(on_add_cathchable);
    app.observe(on_remove_cathchable);
    app.observe(on_enemy_despawn);

    app.run();
}

// #[derive(Event)]
// struct AnimEvent(Vec3);

// fn trigger_animation_in_enemy(
//     mut event_anim: EventReader<AnimEvent>,
//     mut query: Query<(&mut Transform, &mut AnimConfig)>,
// ) {
//     for ev in event_anim.read() {
//         // we expect the Component of type S to be used as a marker Component by only a single entity
//         let (mut transform, mut animation) = query.single_mut();
//
//         transform.translation = ev.0;
//
//         // we create a new timer when the animation is triggered
//         animation.frame_timer = AnimConfig::timer_from_fps(animation.fps);
//     }
// }

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimConfig, &mut TextureAtlas)>) {
    for (mut config, mut atlas) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_i {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_i;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimConfig::timer_from_fps(config.fps);
            }
        }
    }
}
