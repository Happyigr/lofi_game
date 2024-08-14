use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct AnimConfig {
    pub timer: Timer,
    pub first_i: usize,
    pub last_i: usize,
    pub fps: u8,
}

impl AnimConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_i: first,
            last_i: last,
            fps,
            timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimConfig, &mut TextureAtlas)>,
) {
    for (entity, mut config, mut atlas) in &mut query {
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
